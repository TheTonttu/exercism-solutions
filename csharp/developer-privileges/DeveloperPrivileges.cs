using System.Collections.Generic;
using NameIdentityMap = System.Collections.Generic.Dictionary<string, Identity>;

public class Authenticator
{
    public Identity Admin { get; } = new()
    {
        Email = "admin@ex.ism",
        FacialFeatures = new() { EyeColor = "green", PhiltrumWidth = 0.9m },
        NameAndAddress = new[] { "Chanakya", "Mumbai", "India" }
    };

    public IDictionary<string, Identity> Developers { get; } = new NameIdentityMap() {
        {
            "Bertrand",
            new Identity() {
                Email = "bert@ex.ism",
                FacialFeatures = new() { EyeColor = "blue", PhiltrumWidth = 0.8m },
                NameAndAddress = new[] { "Bertrand", "Paris", "France" }
            }
        },
        {
            "Anders",
            new Identity() {
                Email = "anders@ex.ism",
                FacialFeatures = new() { EyeColor = "brown", PhiltrumWidth = 0.85m },
                NameAndAddress = new[] { "Anders", "Redmond", "USA" }
            }
        }

    };
}

//**** please do not modify the FacialFeatures class ****
public class FacialFeatures
{
    public string EyeColor { get; set; }
    public decimal PhiltrumWidth { get; set; }
}

//**** please do not modify the Identity class ****
public class Identity
{
    public string Email { get; set; }
    public FacialFeatures FacialFeatures { get; set; }
    public IList<string> NameAndAddress { get; set; }
}
