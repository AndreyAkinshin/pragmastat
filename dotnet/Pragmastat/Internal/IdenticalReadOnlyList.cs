using System.Collections;

namespace Pragmastat.Internal;

internal class IdenticalReadOnlyList<T>(int count, T value) : IReadOnlyList<T>
{
    public int Count { get; } = count;
    private T Value { get; } = value;

    public IEnumerator<T> GetEnumerator() => new IdenticalEnumerator(Count, Value);
    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
    public T this[int index] => Value;

    private class IdenticalEnumerator(int size, T value) : IEnumerator<T>
    {
        private int counter;

        public bool MoveNext() => counter++ < size;
        public void Reset() => counter = 0;
        public T Current { get; } = value;
        object? IEnumerator.Current => Current;

        public void Dispose()
        {
        }
    }
}