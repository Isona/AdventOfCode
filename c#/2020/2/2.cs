using System;
using System.Text.RegularExpressions;

class Program
{
    static void Main()
    {
        string[] lines = System.IO.File.ReadAllLines(@"2input.txt");
        Regex regex = new Regex(@"^(\d+)-(\d+) ([a-z]): ([a-z]+)$");
        int policyMatchCount = 0;

        foreach (string line in lines)
        {
            Match match = regex.Match(line);
            int policyMin = int.Parse(match.Groups[1].Value);
            int policyMax = int.Parse(match.Groups[2].Value);
            char policyChar = char.Parse(match.Groups[3].Value);
            string password = match.Groups[4].Value;

            int policyCharCount = 0;
            foreach (char c in password)
                if (c == policyChar) policyCharCount++;

            if (policyCharCount >= policyMin && policyCharCount <= policyMax)
                policyMatchCount ++;
        }

        Console.WriteLine($"{policyMatchCount} passwords matched");
    }
}