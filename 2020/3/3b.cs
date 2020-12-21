using System;

class Program
{
    static void Main()
    {
        string[] lines = System.IO.File.ReadAllLines(@"3input.txt");
        uint solution = 1;
        solution *= ski(lines, 1, 1);
        solution *= ski(lines, 3, 1);
        solution *= ski(lines, 5, 1);
        solution *= ski(lines, 7, 1);
        solution *= ski(lines, 1, 2);

        Console.WriteLine($"The solution is {solution}, that's a lot of trees!");
    }

    static uint ski(string[] lines, int xDist, int yDist)
    {
        uint treesHit = 0;
        int lineLen = lines[0].Length;
        int currentX = 0;

        for (int i = 0; i < lines.Length; i+=yDist)
        {
            if (lines[i][currentX] == '#')
                treesHit ++;
            currentX += xDist;
            currentX %= lineLen;
        }
        Console.WriteLine($"For {xDist}, {yDist} we hit {treesHit} trees");
        return treesHit;
    }
}