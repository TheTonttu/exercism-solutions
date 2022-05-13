using System;

public class Orm : IDisposable
{
    private readonly Database _database;

    private bool _isDisposed;

    public Orm(Database database)
    {
        _database = database;
    }

    public void Begin()
    {
        _database.BeginTransaction();
    }

    public void Write(string data)
    {
        try
        {
            _database.Write(data);
        } catch (InvalidOperationException)
        {
            _database.Dispose();
        }
    }

    public void Commit()
    {
        try
        {
            _database.EndTransaction();
        } catch (InvalidOperationException)
        {
            _database.Dispose();
        }
    }

    #region IDisposable Members

    protected virtual void Dispose(bool disposingManaged)
    {
        if (_isDisposed) { return; }

        if (disposingManaged)
        {
            _database.Dispose();
        }
        _isDisposed = true;
    }

    public void Dispose()
    {
        // Do not change this code. Put cleanup code in 'Dispose(bool disposing)' method
        Dispose(disposingManaged: true);
        GC.SuppressFinalize(this);
    }

    #endregion IDisposable Members
}
