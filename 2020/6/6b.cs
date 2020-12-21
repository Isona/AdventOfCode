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
        int numInGroup = 0;
        Dictionary<char, int> groupAnswers = new Dictionary<char, int>();

        foreach (string line in lines)
        {
            if (line == "")
            {
                int groupSum = 0;
                foreach(KeyValuePair<char, int> entry in groupAnswers)
                {
                    if (entry.Value == numInGroup)
                        groupSum ++;
                    // do something with entry.Value or entry.Key
                }
                answerSum += groupSum;
                Console.WriteLine(string.Join(Environment.NewLine, groupAnswers.Select(a => $"{a.Key}: {a.Value}")));
                Console.WriteLine($"There were {numInGroup} people, and the groupSum was {groupSum}");
                groupAnswers = new Dictionary<char, int>();
                numInGroup = 0;
            }
            else
            {
                numInGroup++;

                foreach (char c in line)
                {
                    if (groupAnswers.ContainsKey(c))
                    {
                        groupAnswers[c]++;
                    }
                    else
                    {
                        groupAnswers.Add(c, 1);
                    }
                }
            }
        }

        Console.WriteLine($"The answer is {answerSum}");
    }
}