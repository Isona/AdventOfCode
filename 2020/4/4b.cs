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
        if (!(pptFields.ContainsKey("byr") &&
            pptFields.ContainsKey("iyr") &&
            pptFields.ContainsKey("eyr") &&
            pptFields.ContainsKey("hgt") &&
            pptFields.ContainsKey("hcl") &&
            pptFields.ContainsKey("ecl") &&
            pptFields.ContainsKey("pid")))
            return false;

        int birthYear = Int32.Parse(pptFields["byr"]);
        if (birthYear < 1920 || birthYear > 2002)
            return false;

        int issueYear = Int32.Parse(pptFields["iyr"]);
        if (issueYear < 2010 || issueYear > 2020)
            return False;

        int expirationYear = Int32.Parse(pptFields["eyr"]);
        if (expirationYear <2020 || issueYear > 2030)
            return False;
        return true;
    }
}