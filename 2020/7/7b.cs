using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main()
    {
        string[] lines = System.IO.File.ReadAllLines(@"7input.txt");
        Dictionary<string, List<Tuple<string,int>>> caseRules = new Dictionary<string, List<Tuple<string,int>>>();

        foreach (string line in lines)
        {
            if (line.Contains("no other"))
                continue;

            string[] words = line.Split(' ');

            string mainCaseColor = words[0]+words[1];
            caseRules[mainCaseColor] = new List<string>();
            //caseRules[mainCaseColor].Add(words[5]+words[6]);
            for (int i = 5; (i+1 < words.Length); i+=4)
            {
                caseRules[mainCaseColor].Add(words[i]+words[i+1]);
            }
            //string[] children = 
        }

        HashSet<string> containers = getContainers(caseRules, "shinygold");

        foreach(string container in containers)
        {
            Console.WriteLine(container);
        }

        Console.WriteLine(containers.Count);
        //Console.WriteLine(string.Join(Environment.NewLine, caseRules.Select(a => $"{a.Key}: {String.Join("; ", a.Value)}")));

        //Console.WriteLine($"The answer is {answerSum}");
    }

    static HashSet<string> getContainers(Dictionary<string, List<Tuple<string,int>>> rules, string caseColour)
    {
        HashSet<string> containers = new HashSet<string>();

        foreach(KeyValuePair<string, List<Tuple<string,int>>> rule in rules)
        {
            if (rule.Value.Contains(caseColour))
            {
                containers.Add(rule.Key);
                containers.UnionWith(getContainers(rules,rule.Key));
            }
        }

        return(containers);
    }
}