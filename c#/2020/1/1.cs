using System;

class Program
{
    static void Main()
    {
        string[] lines = System.IO.File.ReadAllLines(@"1input.txt");
        foreach (string first in lines)
        {
            foreach (string second in lines)
            {
                int sum = int.Parse(first) + int.Parse(second);
                if (sum == 2020)
                {
                    Console.WriteLine($"{first} + {second} = 2020");
                    Console.WriteLine(int.Parse(first) * int.Parse(second));
                    System.Environment.Exit(0);
                }
            }
        }
    }
}