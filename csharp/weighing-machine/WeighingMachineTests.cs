using System;
using Xunit;
using Exercism.Tests;

public class WeighingMachineTests
{
    [Fact]
    [Task(1)]
    public void Get_Precision()
    {
        var wm = new WeighingMachine(precision: 3);
        Assert.Equal(3, wm.Precision);
    }

    [Fact]
    [Task(2)]
    public void Set_weight_and_get_weight()
    {
        var wm = new WeighingMachine(precision: 3);
        wm.Weight = 60.567;
        Assert.Equal(60.567, wm.Weight, precision:3);
    }

    [Fact]
    [Task(3)]
    public void Negative_weight_is_invalid()
    {
        var wm = new WeighingMachine(precision: 3);
        Assert.Throws<ArgumentOutOfRangeException>(() => wm.Weight = -10);
    }

    [Fact]
    [Task(4)]
    public void Apply_tare_adjustment_and_get_display_weight()
    {
        var wm = new WeighingMachine(precision: 3);
        wm.Weight = 100.770;
        wm.TareAdjustment = 10;
        Assert.Equal("90.770 kg", wm.DisplayWeight);
    }

    [Fact]
    [Task(5)]
    public void Apply_Default_tare_adjustment_and_get_display_weight()
    {
        var wm = new WeighingMachine(precision: 3);
        wm.Weight = 100.567;
        Assert.Equal("95.567 kg", wm.DisplayWeight);
    }

    [Fact]
    [Task(6)]
    public void Apply_negative_tare_adjustment_and_get_display_weight()
    {
        var wm = new WeighingMachine(precision: 3);
        wm.Weight = 100;
        wm.TareAdjustment = -10.567;
        Assert.Equal("110.567 kg", wm.DisplayWeight);
    }

    [Fact]
    [Task(6)]
    public void Apply_large_tare_adjustment_to_allow_negative_display_weight()
    {
        var wm = new WeighingMachine(precision: 3);
        wm.Weight = 100;
        wm.TareAdjustment = 110.567;
        Assert.Equal("-10.567 kg", wm.DisplayWeight);
    }

    #region Extra tests

    [Fact]
    public void Can_format_max_weight()
    {
        var wm = new WeighingMachine(precision: 5);
        wm.Weight = double.MaxValue;
        Assert.Equal("1797693134862315708145274237317043567980705675258449965989174768031572607800285387605895586327668" +
            "7817154045895351438246423432132688946418276846754670353751698604991057655128207624549009038932894407586850" +
            "8455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368.00000 kg", wm.DisplayWeight);
    }

    #endregion
}
