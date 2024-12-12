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
                foreach(string third in lines)
                {
                    int sum = int.Parse(first) + int.Parse(second) + int.Parse(third);
                    if (sum == 2020)
                    {
                        Console.WriteLine($"{first} + {second} + {third} = 2020");
                        Console.WriteLine(int.Parse(first) * int.Parse(second) * int.Parse(third));
                        System.Environment.Exit(0);
                    }
                }
            }
        }
    }
}