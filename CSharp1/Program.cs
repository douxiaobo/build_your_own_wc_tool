namespace CSharp1;
using System;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;

class Program
{
    static void Main(string[] args)
    {
        string command = string.Empty;
        string file_name = string.Empty;
        // long byteCount = 0;

        if (args.Length > 0)
        {
            foreach (var arg in args)
            {
                if (arg.StartsWith("-"))
                {
                    command += arg.Substring(1);
                }
                else if (File.Exists(arg))
                {
                    file_name = arg;
                }
            }
        }
        else
        {
            Console.WriteLine("Invalid Command.");
            Environment.Exit(1);
        }
        Console.WriteLine("command:{0},file_name:{1}", command, file_name);

        // string contents = string.Empty;
        if (string.IsNullOrEmpty(file_name))
        {
            // using (StreamReader reader = new StreamReader(Console.In))
            // {
            //     string line;
            //     while ((line = reader.ReadLine()) != null && line.ToLower() != "exit")
            //     {
            //         contents += line + Environment.NewLine;
            //         // 对于UTF-8编码，每个字符可能占用1到4个字节，但这里假设每个字符1个字节进行近似计算
            //         byteCount += Encoding.UTF8.GetByteCount(line) + Environment.NewLine.Length;
            //     }
            // }
            string input = Console.In.ReadToEnd();
            long byteCount = Encoding.UTF8.GetByteCount(input); // 字节数
            var inputLines = input.Split(new[] { Environment.NewLine }, StringSplitOptions.None);
            AnalyzeContent(inputLines, byteCount);
        }
        else
        {
            try
            {
                // 读取文件内容并分析
                var fileInfo = new FileInfo(file_name);
                long byteCount = fileInfo.Length; // 字节数
                var fileContent = File.ReadAllLines(file_name);
                AnalyzeContent(fileContent, byteCount);
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error read file: {ex.Message}");
                Environment.Exit(1);
            }
        }

        // // 读取文件的所有字节，并计算字节数
        // byte[] fileBytes = File.ReadAllBytes(file_name);
        // long byteCount = fileBytes.Length;
        // Console.WriteLine($"Byte Count:{byteCount}");   //342190    Correct

        // // 计算行数
        // int lineCount = contents.Split(new[] { Environment.NewLine }, StringSplitOptions.None).Length;
        // Console.WriteLine($"Line Count:{lineCount}");   //7146  Correct:7145    -l

        // // 计算单词数（这里假设单词由空格、制表符、换行符等分隔）
        // Regex wordRegex = new Regex(@"\b\w+\b");
        // int wordCount = wordRegex.Matches(contents).Count;
        // Console.WriteLine($"Wold Count:{wordCount}");   //59792 Incorrect

        // // 计算字符数（不包括换行符）
        // int charCount = contents.Length - Environment.NewLine.Length * (lineCount - 1); // 减去多余的换行符
        // Console.WriteLine($"Char Count:{charCount}");   //332146    Incorrect

        // 读取文件并计算

        // int lineCount = File.ReadAllLines(file_name).Length; // 文件行数
        // int wordCount = File.ReadLines(file_name).Sum(line => line.Split(new char[] { ' ', '\t', '\r', '\n' }, StringSplitOptions.RemoveEmptyEntries).Length); // 文件单词数
        // int charCount = File.ReadAllText(file_name).Length; // 文件字符数

        // // 打印结果
        // Console.WriteLine($"Byte Count: {byteCount}");          //342190    Correct
        // Console.WriteLine($"Line Count: {lineCount}");          //7145      Correct
        // Console.WriteLine($"Word Count: {wordCount}");          //58164     Correct
        // Console.WriteLine($"Character Count: {charCount}");     //339291    Correct:339292

    }
    // static string ReadFromStandardInput()
    // {
    //     StringBuilder builder = new StringBuilder();
    //     string? line = null;
    //     while ((line = Console.ReadLine()) != null)
    //     {
    //         builder.AppendLine(line);
    //     }
    //     return builder.ToString();
    // }

    static void AnalyzeContent(string[] lines, long byteCount)
    {
        int lineCount = lines.Length; // 行数
        int wordCount = lines.Sum(line => line.Split(new char[] { ' ', '\t', '\r', '\n' }, StringSplitOptions.RemoveEmptyEntries).Length); // 单词数
        int charCount = lines.Sum(line => line.Length) - lines.Length; // 字符数（减去换行符的数量）

        // 打印结果
        Console.WriteLine($"Byte Count: {byteCount}");      //342190    Correct
        Console.WriteLine($"Line Count: {lineCount}");      //7145      Correct
        Console.WriteLine($"Word Count: {wordCount}");      //58164     Correct
        Console.WriteLine($"Character Count: {charCount}");     //317856    Correct:339292
    }
}

// Last login: Sun Jun  9 18:59:02 on ttys001
// douxiaobo@192 CSharp1 % dotnet new console --framework net8.0 --use-program-main
// 已成功创建模板“控制台应用”。

// 正在处理创建后操作...
// 正在还原 /Users/douxiaobo/Documents/Coding/Practice in Coding/build_your_own_wc_tool/CSharp1/CSharp1.csproj:
//   Determining projects to restore...
//   Restored /Users/douxiaobo/Documents/Coding/Practice in Coding/build_your_own_wc_tool/CSharp1/CSharp1.csproj (in 1.58 sec).
// 已成功还原。


// douxiaobo@192 CSharp1 % zed .
// douxiaobo@192 CSharp1 %

// douxiaobo@192 CSharp1 % dotnet run -abc test.txt
// command:abc, file_name:test.txt
// Byte Count: 342190
// Line Count: 7145
// Word Count: 58164
// Character Count: 317856
// douxiaobo@192 CSharp1 % dotnet build
// 适用于.NET MSBuild 版本 17.8.5+b5265ef37
//   Determining projects to restore...
//   All projects are up-to-date for restore.
//   CSharp1 -> /Users/douxiaobo/Documents/Coding/Practice in Coding/build_your_own_wc_tool/CSharp1/bin/Debug/net8.0/CSharp1.dll

// 已成功生成。
//     0 个警告
//     0 个错误

// 已用时间 00:00:00.41
// douxiaobo@192 CSharp1 % cat test.txt | ./bin/Debug/net8.0/Csharp -abc
// zsh: no such file or directory: ./bin/Debug/net8.0/Csharp
// douxiaobo@192 CSharp1 % cat test.txt | ./bin/Debug/net8.0/Csharp1 -abc
// command:abc, file_name:
// Byte Count: 342190
// Line Count: 7146
// Word Count: 58164
// Character Count: 325001
// douxiaobo@192 CSharp1 %
