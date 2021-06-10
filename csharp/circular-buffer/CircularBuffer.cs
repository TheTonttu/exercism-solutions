using System;

public class CircularBuffer<T>
{
    private int _headIndex;
    private int _tailIndex;
    private T[] _innerArray;

    public int Capacity => _innerArray.Length;
    public int Size { get; private set; }
    public bool IsFull => Size >= Capacity;

    public CircularBuffer(int capacity)
    {
        _innerArray = new T[capacity];
    }

    public T Read()
    {
        if (Size == 0)
        {
            throw new InvalidOperationException("No data available.");
        }
        T oldestItem = _innerArray[_headIndex];
        _headIndex = (_headIndex + 1) % Capacity;
        Size -= 1;

        return oldestItem;
    }

    public void Write(T value)
    {
        if (IsFull)
        {
            throw new InvalidOperationException("Buffer is full.");
        }
        _innerArray[_tailIndex] = value;
        _tailIndex = (_tailIndex + 1) % Capacity;
        Size += 1;
    }

    public void Overwrite(T value)
    {
        if (!IsFull)
        {
            Write(value);
            return;
        }
        _innerArray[_tailIndex] = value;
        _headIndex = (_headIndex + 1) % Capacity;
        _tailIndex = (_tailIndex + 1) % Capacity;
    }

    public void Clear()
    {
        _headIndex = 0;
        _tailIndex = 0;
        Size = 0;
    }
}