using System;

abstract class Character
{
    private readonly string _characterType;

    protected readonly int _baseDamage;

    protected Character(string characterType, int baseDamage)
    {
        _characterType = characterType;
        _baseDamage = baseDamage;
    }

    public abstract int DamagePoints(Character target);

    public virtual bool Vulnerable() => false;

    public override string ToString() => $"Character is a {_characterType}";
}

class Warrior : Character
{
    private const int CriticalDamage = 10;

    public Warrior() : base(nameof(Warrior), 6) { }

    public override int DamagePoints(Character target) =>
        target.Vulnerable()
        ? CriticalDamage
        : _baseDamage;
}

class Wizard : Character
{
    private const int SpellDamage = 12;

    private bool _isSpellPrepared;

    public Wizard() : base(nameof(Wizard), 3) { }

    public override int DamagePoints(Character target) =>
        _isSpellPrepared
        ? SpellDamage
        : _baseDamage;

    public void PrepareSpell() => _isSpellPrepared = true;

    public override bool Vulnerable() => !_isSpellPrepared;
}
