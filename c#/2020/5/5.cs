using System;
using System.Linq;

class Program
{
    static void Main()
    {
        string[] lines = System.IO.File.ReadAllLines(@"5input.txt");
        int highestValue = 0;
        int lowestValue = Int32.MaxValue;
        int seatSum = 0;

        foreach (string line in lines)
        {

            string boolLine = line.Replace("F", "0");
            boolLine = boolLine.Replace("B", "1");
            boolLine = boolLine.Replace("L", "0");
            boolLine = boolLine.Replace("R", "1");
            int seatValue = Convert.ToInt32(boolLine, 2);

            if (seatValue > highestValue)
                highestValue = seatValue;
            if (seatValue < lowestValue)
                lowestValue = seatValue;

            seatSum += seatValue;
        }

        Console.WriteLine($"The highest seat value was {highestValue}");
        Console.WriteLine($"The lowest seat value was {lowestValue}");
        int totalValue = Enumerable.Range(lowestValue, (highestValue-lowestValue+1)).Sum();
        int mySeat = totalValue - seatSum;
        Console.WriteLine($"My seat is {mySeat}");
    }
}