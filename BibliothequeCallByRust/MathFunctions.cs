using System;
using System.Runtime.InteropServices;

namespace BibliothequeCallByRust;

public static class MathFunctions
{
    // Est appelé par RustCallCSharp
    [UnmanagedCallersOnly(EntryPoint = "add_numbers")]
    public static int AddNumbers(int a, int b)
    {
        Console.WriteLine("Hello from C#");
        return a + b;
    }
}