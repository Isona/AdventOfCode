using System;
using System.Collections.Generic;

class Program
{
    static void Main()
    {
        string[] lines = System.IO.File.ReadAllLines(@"4input.txt");
        Passport currPassport = new Passport();
        int validCount = 0;
        foreach (string line in lines)
        {
            if (line == "")
            {
                if (currPassport.isValid())
                    validCount++;
                currPassport = new Passport();
            }
            else
            {
                string[] fields=line.Split(' ');
                foreach (string field in fields)
                {
                    string[] splitField = field.Split(':');
                    currPassport.pptFields.Add(splitField[0], splitField[1]);
                }
            }
        }

        if (currPassport.isValid())
            validCount++;
        Console.WriteLine($"There were {validCount} valid passports!");
    }
}

class Passport
{
    public Dictionary<string, string> pptFields = new Dictionary<string, string>();

    public Passport()
    {
    }

    public bool isValid()
    {
        return (pptFields.ContainsKey("byr") &&
            pptFields.ContainsKey("iyr") &&
            pptFields.ContainsKey("eyr") &&
            pptFields.ContainsKey("hgt") &&
            pptFields.ContainsKey("hcl") &&
            pptFields.ContainsKey("ecl") &&
            pptFields.ContainsKey("pid"));
    }
}