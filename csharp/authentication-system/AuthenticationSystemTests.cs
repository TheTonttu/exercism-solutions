using System;
using System.Collections.Generic;
using Xunit;
using Exercism.Tests;
using System.Linq;
using System.Reflection;

public class AuthenticationSystemTests
{
    [Fact]
    [Task(4)]
    public void GetAdmin()
    {
        var admin = new Identity { EyeColor = "green", Email = "admin@ex.ism" };
        var authenticator = new Authenticator(admin);
        Assert.Equal(admin, authenticator.Admin);
    }

    [Fact]
    [Task(5)]
    public void GetDevelopers()
    {
        var authenticator = new Authenticator(new Identity { EyeColor = "green", Email = "admin@ex.ism" });
        var devs = authenticator.GetDevelopers() as IDictionary<string, Identity>;
        bool?[] actual = { devs != null, devs?.Count == 2, devs?["Anders"].EyeColor == "brown" };
        bool?[] expected = { true, true, true };
        Assert.Equal(expected, actual);
    }

    #region Extra tests

    [Fact]
    public void IdentityIsReadOnly()
    {
        bool allIdentityPropertySettersAreInitOnly = typeof(Identity)
            .GetProperties()
            .All(p => IsPropertySetterInitOnly(p));

        Assert.True(allIdentityPropertySettersAreInitOnly);
    }

    [Fact]
    public void CannotModifyAdmin()
    {
        var adminProperty = typeof(Authenticator).GetProperty(nameof(Authenticator.Admin));

        bool canSetAdmin = adminProperty.CanWrite;
        bool adminPropertyReturnTypeIsIdentity = adminProperty
            .GetGetMethod()
            // Identity is readonly => admin is readonly
            .ReturnType == typeof(Identity);

        Assert.False(canSetAdmin);
        Assert.True(adminPropertyReturnTypeIsIdentity);
    }

    [Fact]
    public void CannotReplaceDevelopers()
    {
        var authenticator = new Authenticator(new Identity());
        IReadOnlyDictionary<string, Identity> readOnlyDevelopers = authenticator.GetDevelopers();
        (string originalDevKey, _) = readOnlyDevelopers.First();

        var modifiableDevelopers = authenticator.GetDevelopers() as IDictionary<string, Identity>;
        var replacementDev = new Identity() { Email = "replaced", EyeColor = "yes" };

        Action failingDevReplaceAttempt = () => modifiableDevelopers[originalDevKey] = replacementDev;
        Assert.Throws<NotSupportedException>(failingDevReplaceAttempt);
    }

    private static bool IsPropertySetterInitOnly(PropertyInfo property)
    {
        if (!property.CanWrite) { return false; }

        var setMethod = property.SetMethod;
        var requiredCustomModifiers = setMethod.ReturnParameter.GetRequiredCustomModifiers();
        return requiredCustomModifiers.Contains(typeof(System.Runtime.CompilerServices.IsExternalInit));
    }

    #endregion
}
