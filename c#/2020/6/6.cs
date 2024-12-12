using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        List<string> lines = new List<string>(System.IO.File.ReadAllLines(@"6input.txt"));
        lines.Add(""); //Add an extra newline to make sure the last group counts
        int answerSum = 0;
        //Dictionary<char, int> groupAnswers = new Dictionary<char, int>();
        HashSet<char> groupAnswers = new HashSet<char>();

        foreach (string line in lines)
        {
            if (line == "")
            {
                answerSum += groupAnswers.Count;
                //Console.WriteLine($"There were {numInGroup} people, and the groupSum was {groupSum}");
                groupAnswers = new HashSet<char>();
            }
            else
            {
                foreach (char c in line)
                {
                    groupAnswers.Add(c);
                }
            }
        }

        Console.WriteLine($"The answer is {answerSum}");
    }
}