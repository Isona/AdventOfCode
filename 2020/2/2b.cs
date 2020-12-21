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
            int policyFirst = int.Parse(match.Groups[1].Value);
            int policySecond = int.Parse(match.Groups[2].Value);
            char policyChar = char.Parse(match.Groups[3].Value);
            string password = match.Groups[4].Value;



            if ((password[policyFirst-1] == policyChar || password[policySecond-1] == policyChar) 
                && (password[policyFirst-1] != password[policySecond-1]))
                policyMatchCount ++;
        }

        Console.WriteLine($"{policyMatchCount} passwords matched");
    }
}