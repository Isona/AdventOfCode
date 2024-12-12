using System;

class Program
{
    static void Main()
    {
        string[] lines = System.IO.File.ReadAllLines(@"3input.txt");
        int treesHit = 0;
        int lineLen = lines[0].Length;
        int currentX = 0;

        foreach (string line in lines)
        {
            if (line[currentX] == '#')
                treesHit ++;
            currentX += 3;
            currentX %= lineLen;

        }

        Console.WriteLine($"We hit {treesHit} trees!");
    }
}