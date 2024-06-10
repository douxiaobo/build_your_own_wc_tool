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
    static string command=string.Empty;

    static void Main(string[] args)
    {
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
        if (file_name == string.Empty)  //if(string.IsNullOrEmpty(file_name))
        {
            AnalyzeStandardInput();
        }
        Console.WriteLine(result(command));
    }
    static void AnalyzeFile(string filePath)
    {
        // 读取文件内容并分析
        // var fileInfo = new FileInfo(filePath);
        // byteCount = fileInfo.Length; // 字节数
        // var fileContent = File.ReadAllLines(filePath);
        // AnalyzeContent(string.Join("\n", fileContent), fileContent);

        // 读取文件并分析
        file_name = filePath;
        byteCount = new FileInfo(filePath).Length; // 获取文件字节数
        AnalyzeContent(File.ReadAllText(filePath));
    }

    static void AnalyzeStandardInput()
    {
        // // 从标准输入读取所有行
        // string input = Console.In.ReadToEnd();
        // byteCount = Encoding.UTF8.GetByteCount(input); // 字节数
        // var inputLines = input.Split(new[] { Environment.NewLine }, StringSplitOptions.None);
        // AnalyzeContent(input, inputLines);

        // 从标准输入读取所有内容
        // string input = Console.In.ReadToEnd();
        // byteCount = Encoding.UTF8.GetByteCount(input); // 计算输入的字节数
        // AnalyzeContent(input);

        // string line=string.Empty;
        // var inputBuilder = new StringBuilder();
        // byteCount = 0;
        // while ((line = Console.ReadLine()) != null)
        // {
        //     inputBuilder.Append(line);
        //     inputBuilder.Append(Environment.NewLine); // 确保保留换行符
        //     byteCount += Encoding.UTF8.GetByteCount(line) + Environment.NewLine.Length; // 更新字节数
        // }
        // AnalyzeContent(inputBuilder.ToString());

        // string inputLine;
        // var inputBuilder = new StringBuilder();
        // while ((inputLine = Console.ReadLine()) != null)
        // {
        //     inputBuilder.AppendLine(inputLine); // 包括换行符
        //     byteCount += Encoding.UTF8.GetByteCount(inputLine) + 1; // 包括换行符的字节数
        // }
        // AnalyzeContent(inputBuilder.ToString());

        // 获取 Console.In 的底层 Stream
        Stream inputStream = Console.OpenStandardInput();
        int input;
        List<byte> inputBytes = new List<byte>();
        while ((input = inputStream.ReadByte()) != -1) // 逐字节读取直到输入结束
        {
            inputBytes.Add((byte)input);
        }

        // 计算字节数
        byteCount = inputBytes.Count;

        // 将字节转换为字符串
        var memoryStream = new MemoryStream(inputBytes.ToArray());
        using (var reader = new StreamReader(memoryStream, Encoding.UTF8))
        {
            string inputContent = reader.ReadToEnd();
            AnalyzeContent(inputContent);
        }
    }
    // static void AnalyzeContent(string content, string[] lines)
    // {
    //     lineCount = lines.Length; // 行数
    //     wordCount = lines.Sum(line => line.Split(new char[] { ' ', '\t', '\r', '\n' }, StringSplitOptions.RemoveEmptyEntries).Length); // 单词数
    //     // charCount = lines.Sum(line => line.Length) - lines.Length; // 字符数（减去换行符的数量）
    //     // 正确的字符数计算，包括换行符
    //     charCount = content.Length;
    //     contents = content;
    // }

    static void AnalyzeContent(string content)
    {
        // 计算行数、单词数和字符数
        lineCount = content.Split(new[] { Environment.NewLine }, StringSplitOptions.None).Length;
        wordCount = content.Split(new[] { ' ', '\t', '\r', '\n' }, StringSplitOptions.RemoveEmptyEntries).Length;
        charCount = content.Length;
    }

    static string result(string command)
    {
        if (string.IsNullOrEmpty(command))
        {
            command = "lwc";
        }
        // Console.WriteLine("Command:"+command);
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
         result = result.TrimEnd('\t'); // 移除尾部的制表符
        return result + (!string.IsNullOrEmpty(file_name) ? $"\t{file_name}" : "");
        // return result + file_name;
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

// douxiaobo@192 Csharp2 % cat test.txt | ./bin/Debug/net8.0/Csharp2 -clwm
// Command:clwm
// 	342190	7146	58164	339291	
// douxiaobo@192 Csharp2 % ./bin/Debug/net8.0/Csharp2 -clwm                        //这行是死循环

// douxiaobo@192 Csharp2 % ./bin/Debug/net8.0/Csharp2 -clwm test.txt
// Command:clwm
// 	342190	7146	58164	339291		test.txt
// douxiaobo@192 Csharp2 % ./bin/Debug/net8.0/Csharp2                           //这行是死循环

// douxiaobo@192 Csharp2 % ./bin/Debug/net8.0/Csharp2 test.txt
// Command:lwc
// 	7146	58164	342190		test.txt