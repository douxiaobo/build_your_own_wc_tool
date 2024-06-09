namespace Csharp2;
using System;
using System.IO;
using System.Linq;
using System.Text;

class Program
{
    static long byteCount;
    static int lineCount;
    static int wordCount;
    static int charCount;
    static string contents = string.Empty;
    static string file_name = string.Empty;
    static void Main(string[] args)
    {
        string command = string.Empty;
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
                    AnalyzeFile(arg);
                }
            }
        }
        else
        {
            AnalyzeStandardInput();
        }
        if (string.IsNullOrEmpty(command))
        {
            command = "lwc";
        }
        Console.WriteLine(result(command));
        // string complete = result(command);
        // if (!string.IsNullOrEmpty(complete))
        // {
        //     Console.WriteLine(complete);
        // }
    }
    static void AnalyzeFile(string filePath)
    {
        // 读取文件内容并分析
        var fileInfo = new FileInfo(filePath);
        byteCount = fileInfo.Length; // 字节数
        var fileContent = File.ReadAllLines(filePath);
        AnalyzeContent(string.Join("\n", fileContent), fileContent);
    }

    static void AnalyzeStandardInput()
    {
        // 从标准输入读取所有行
        string input = Console.In.ReadToEnd();
        byteCount = Encoding.UTF8.GetByteCount(input); // 字节数
        var inputLines = input.Split(new[] { Environment.NewLine }, StringSplitOptions.None);
        AnalyzeContent(input, inputLines);
    }
    static void AnalyzeContent(string content, string[] lines)
    {
        lineCount = lines.Length; // 行数
        wordCount = lines.Sum(line => line.Split(new char[] { ' ', '\t', '\r', '\n' }, StringSplitOptions.RemoveEmptyEntries).Length); // 单词数
        // charCount = lines.Sum(line => line.Length) - lines.Length; // 字符数（减去换行符的数量）
        // 正确的字符数计算，包括换行符
        charCount = content.Length;
        contents = content;
    }
    static string result(string command)
    {
        string result = string.Empty;
        result += "\t";
        foreach (var ch in command)
        {
            switch (ch)
            {
                case 'c':
                    result += byteCount + "\t";
                    break;
                case 'l':
                    result += lineCount + "\t";
                    break;
                case 'w':
                    result += wordCount + "\t";
                    break;
                case 'm':
                    result += charCount + "\t";
                    break;
                default:
                    result += $"Unknown command '{ch}\t";
                    break;
            }
        }
        return result + file_name;
        // if (!string.IsNullOrEmpty(contents))
        // {
        //     return result + (string.IsNullOrEmpty(contents) ? "" : file_name);
        // }
        // else
        // {
        //     return "";
        // }
    }
}

// Last login: Sun Jun  9 20:41:43 on ttys001
// douxiaobo@192 Csharp2 % dotnet new console --framework net8.0 --use-program-main
// 已成功创建模板“控制台应用”。

// 正在处理创建后操作...
// 正在还原 /Users/douxiaobo/Documents/Coding/Practice in Coding/build_your_own_wc_tool/Csharp2/Csharp2.csproj:
//   Determining projects to restore...
//   Restored /Users/douxiaobo/Documents/Coding/Practice in Coding/build_your_own_wc_tool/Csharp2/Csharp2.csproj (in 6.59 sec).
// 已成功还原。


// douxiaobo@192 Csharp2 % zed.
// douxiaobo@192 Csharp2 % dotnet run test.txt -clwm
// 	342190	7145	58164	317856	test.txt
