using System;

public class CircularBuffer<T>
{
    private const int UninitializedIndex = int.MinValue;

    private readonly T[] _innerArray;
    private int _tailIndex = UninitializedIndex;
    private int _headIndex = UninitializedIndex;

    public CircularBuffer(int capacity)
    {
        _innerArray = new T[capacity];

    }

    public T Read()
    {
        if (_headIndex == UninitializedIndex)
        {
            throw new InvalidOperationException("No data available.");
        }
        var firstBufferedItem = _innerArray[_tailIndex];
        _tailIndex++;
        bool wasLastItem = _tailIndex > _headIndex;
        if (wasLastItem)
        {
            // No further data
            _tailIndex = UninitializedIndex;
            _headIndex = UninitializedIndex;
        }
        return firstBufferedItem;
    }

    public void Write(T value)
    {
        if (_headIndex == UninitializedIndex)
        {
            _tailIndex = 0;
            _headIndex = 0;
            _innerArray[_headIndex] = value;
            return;
        }
        int bufferLength = GetBufferLength();
        if (bufferLength >= _innerArray.Length)
        {
            throw new InvalidOperationException("Buffer is full.");
        }
        _headIndex++;
        _innerArray[_headIndex] = value;
    }

    public void Overwrite(T value)
    {
        int bufferLength = GetBufferLength();
        if (bufferLength < _innerArray.Length)
        {
            Write(value);
            return;
        }
        throw new NotImplementedException("You need to implement this function.");
    }

    private int GetBufferLength()
    {
        return (_headIndex - _tailIndex) + 1;
    }

    public void Clear()
    {
        _tailIndex = UninitializedIndex;
        _headIndex = UninitializedIndex;
    }
}