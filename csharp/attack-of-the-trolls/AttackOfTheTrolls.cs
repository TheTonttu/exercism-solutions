using System;

public enum AccountType
{
    Guest,
    User,
    Moderator
}

[Flags]
public enum Permission
{
    None   = 0,
    Read   = 0b001,
    Write  = 0b010,
    Delete = 0b100,
    All    = 0b111 // or ~(~0 << 3)
}

static class Permissions
{
    public static Permission Default(AccountType accountType) => accountType switch
    {
        AccountType.Guest => Permission.Read,
        AccountType.User => Permission.Read | Permission.Write,
        AccountType.Moderator => Permission.All,
        _ => Permission.None
    };

    public static Permission Grant(Permission current, Permission grant) => current | grant;

    public static Permission Revoke(Permission current, Permission revoke) => current & ~revoke;

    public static bool Check(Permission current, Permission check) => current.HasFlag(check);
}
