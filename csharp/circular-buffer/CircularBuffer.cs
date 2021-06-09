using System;

public class CircularBuffer<T>
{
    // The implementation is not really a circular buffer but more like linked list.

    private readonly int _capacity;

    // Could also be done with LinkedList<T>
    private Node<T> _tail = null;
    private Node<T> _head = null;

    private int _length;

    public CircularBuffer(int capacity)
    {
        _capacity = capacity;
    }

    public T Read()
    {
        if (_tail == null)
        {
            throw new InvalidOperationException("No data available.");
        }

        T oldestBufferedItem = _tail.Value;
        _tail = _tail.Next;
        if (_tail == null)
        {
            _head = null;
        }
        _length -= 1;

        return oldestBufferedItem;
    }

    public void Write(T value)
    {
        if (IsBufferFull())
        {
            throw new InvalidOperationException("Buffer is full.");
        }

        if (_head == null)
        {
            _head = new Node<T>(value);
            _tail = _head;
        }
        else
        {
            AddHead(value);
        }
        _length += 1;
    }

    public void Overwrite(T value)
    {
        if (!IsBufferFull())
        {
            Write(value);
            return;
        }

        // Remove tail node
        _tail = _tail.Next;

        AddHead(value);
        _length += 1;
    }

    private void AddHead(T headValue)
    {
        var oldHead = _head;
        var newHead = new Node<T>(headValue);
        // Old head is already included in the tail chain so no need to assign it anywhere explicitly.
        oldHead.Next = newHead;
        _head = newHead;
    }

    private bool IsBufferFull() => _length >= _capacity;

    public void Clear()
    {
        _tail = null;
        _head = null;
        _length = 0;
    }
}

internal class Node<T>
{
    public Node<T> Next { get; set; }

    public T Value { get; }

    public Node(T value)
    {
        Value = value;
    }
}