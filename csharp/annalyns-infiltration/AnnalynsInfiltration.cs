using System;

static class QuestLogic
{
    public static bool CanFastAttack(bool knightIsAwake)
    {
        return !knightIsAwake;
    }

    public static bool CanSpy(bool knightIsAwake, bool archerIsAwake, bool prisonerIsAwake)
    {
        return knightIsAwake || archerIsAwake || prisonerIsAwake;
    }

    public static bool CanSignalPrisoner(bool archerIsAwake, bool prisonerIsAwake)
    {
        return !archerIsAwake && prisonerIsAwake;
    }

    public static bool CanFreePrisoner(bool knightIsAwake, bool archerIsAwake, bool prisonerIsAwake, bool petDogIsPresent)
    {
        return !archerIsAwake
            && IsPrisonerQuiet(prisonerIsAwake, petDogIsPresent)
            && IsKnightOutOfAction(knightIsAwake, petDogIsPresent);
    }

    private static bool IsPrisonerQuiet(bool prisonerIsAwake, bool petDogIsPresent)
    {
        return (prisonerIsAwake || petDogIsPresent);
    }

    private static bool IsKnightOutOfAction(bool knightIsAwake, bool petDogIsPresent)
    {
        return !knightIsAwake || (knightIsAwake && petDogIsPresent);
    }
}
