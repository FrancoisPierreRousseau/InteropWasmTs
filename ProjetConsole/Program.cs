using System.Reflection;
using System.Runtime.InteropServices;

class Program
{
    // Déclaration de la fonction importée depuis la bibliothèque Rust
    [DllImport("InteropCsharpTs.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int add(int a, int b);

    static void Main(string[] args)
    {
        Console.WriteLine(Path.GetDirectoryName(Assembly.GetEntryAssembly().Location));
        int result = add(5, 7);
        Console.WriteLine($"Le résultat de l'addition est : {result}");
    }
}