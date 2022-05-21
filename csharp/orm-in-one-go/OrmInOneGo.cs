using System;

public class Orm
{
    private readonly Database _database;

    public Orm(Database database)
    {
        _database = database;
    }

    public void Write(string data)
    {
        using var db = _database;

        db.BeginTransaction();
        db.Write(data);
        db.EndTransaction();
    }

    public bool WriteSafely(string data)
    {
        try
        {
            _database.BeginTransaction();
            _database.Write(data);
            _database.EndTransaction();
            return true;
        }
        catch (Exception)
        {
            return false;
        }
        finally
        {
            _database.Dispose();
        }
    }
}
