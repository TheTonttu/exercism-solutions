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
        T firstBufferedItem = _innerArray[_tailIndex];
        bool wasHeadWrapped = IsHeadWrapped();
        int nextTailIndex = _tailIndex + 1;
        if (wasHeadWrapped)
        {
            if (nextTailIndex >= _innerArray.Length)
            {
                _tailIndex = 0;
            }
            else
            {
                _tailIndex = nextTailIndex;
            }
        }
        else
        {
            bool wasLastItem = nextTailIndex > _headIndex;
            if (wasLastItem)
            {
                // No further data
                _tailIndex = UninitializedIndex;
                _headIndex = UninitializedIndex;
            }
            else
            {
                _tailIndex = nextTailIndex;
            }
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

        if (IsBufferFull())
        {
            throw new InvalidOperationException("Buffer is full.");
        }

        _headIndex++;
        if (_headIndex >= _innerArray.Length)
        {
            _headIndex = 0;
        }
        _innerArray[_headIndex] = value;
    }

    public void Overwrite(T value)
    {
        if (!IsBufferFull())
        {
            Write(value);
            return;
        }

        if (IsHeadWrapped())
        {
            _tailIndex++;
            _headIndex++;
        }
        else if (_headIndex + 1 >= _innerArray.Length)
        {
            _tailIndex++;
            _headIndex = 0;
        }
        else
        {
            _headIndex = _tailIndex;
            _tailIndex++;
        }

        if (_tailIndex >= _innerArray.Length)
        {
            _tailIndex = 0;
        }
        _innerArray[_headIndex] = value;
    }

    private bool IsBufferFull() => GetBufferLength() >= _innerArray.Length;

    private int GetBufferLength()
    {
        if (IsHeadWrapped())
        {
            return (_tailIndex - _headIndex) + 1;
        }
        return (_headIndex - _tailIndex) + 1;
    }

    private bool IsHeadWrapped()
    {
        return _tailIndex > _headIndex;
    }

    public void Clear()
    {
        _tailIndex = UninitializedIndex;
        _headIndex = UninitializedIndex;
    }
}