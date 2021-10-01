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
        return CanFreeWithDog(archerIsAwake, petDogIsPresent)
            || CanFreeWithoutDog(knightIsAwake, archerIsAwake, prisonerIsAwake);
    }

    private static bool CanFreeWithDog(bool archerIsAwake, bool petDogIsPresent)
    {
        return !archerIsAwake && petDogIsPresent;
    }

    private static bool CanFreeWithoutDog(bool knightIsAwake, bool archerIsAwake, bool prisonerIsAwake)
    {
        return !knightIsAwake && !archerIsAwake && prisonerIsAwake;
    }
}
