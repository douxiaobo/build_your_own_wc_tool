namespace CSharp;
using System;
using System.IO;
using System.Text;
#nullable enable

class Program
{
    static void Main(string[] args)
    {
        // Console.WriteLine("Hello, World!");
        string command=string.Empty;
        string file=string.Empty;
        if(args.Length>0){
            foreach(var arg in args)
            {
                if (arg.StartsWith("-")){
                    command+=arg.Substring(1);
                } else if (arg.Contains(".")){
                    file=arg;
                }
            }
        } else {
            Console.WriteLine("No arguments provided.");
            string loopcontents = ReadFromStandardInput();
            string loopcomplete = result(command, loopcontents, file);
            if (!string.IsNullOrEmpty(loopcomplete))
            {
                Console.WriteLine(loopcomplete);
            }
            return;
        }
        
        if(string.IsNullOrEmpty(command)) {
            command="lwc";
        }

        string contents=string.Empty;
        Console.WriteLine("Command: "+command+", File: "+file);      //OK
        if (string.IsNullOrEmpty(file)) {
            contents=ReadFromStandardInput();
            // string? line=null;
            // while ((line = Console.ReadLine()) != null)  
            // {  
            //     contents += line + Environment.NewLine; // 保留换行符  
            // }  
            // Console.WriteLine(contents); // 输出全部内容  
        } else {
            try
            {
                // 使用File.ReadAllText直接读取文件内容为字符串
                contents = File.ReadAllText(file);
                // Console.WriteLine(contents);
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error reading file: {ex.Message}");
                return;
            }
            // using(FileStream fileStream = new FileStream(file, FileMode.Open)){
            //     byte[] buffer = new byte[1024];
            //     int byteRead;
            //     while ((byteRead = fileStream.Read(buffer, 0, buffer.Length)) > 0) {
            //         contents += Encoding.UTF8.GetString(buffer, 0, byteRead);
            //         Console.WriteLine(contents);
            //     }
            // }
        }
        string complete=result(command,contents,file);
        if (!string.IsNullOrEmpty(complete)) {
            Console.WriteLine(complete);
        }
    }
    static string ReadFromStandardInput()
    {
        StringBuilder builder = new StringBuilder();
        string? line=null;
        while ((line = Console.ReadLine()) != null)
        {
            builder.AppendLine(line);
        }
        return builder.ToString();
    }
    // public static class EncodingExtensions
    // {
    //     public static Encoding UTF8WithoutBOM => new UTF8Encoding(false);
    // }
    static string result(string command, string contents, string file){
        string result=string.Empty;
        result+="\t";
        // bool should_output=true;
        // if(string.IsNullOrEmpty(contents)) {
        //     should_output=false;
        // }
        bool should_output = !string.IsNullOrEmpty(contents);
        foreach (char ch in command) {
            // if(ch=='c'){
            //     result+=contents.Length+"\t";
            // } else (ch=='l'){
            //     result+=contents.Split('\n').Length+"\t";
            // } else (ch=='w'){
            //     result+=contents.split_whitespace().count()+"\t";
            // } else (ch=='m'){
            //     result+=contents.split_words().count()+"\t";
            // }
            switch (ch) // 使用switch-case替代错误的条件语句
            {
            case 'c':{
                // if (!string.IsNullOrEmpty(file))
                // {
                //     result += new FileInfo(file).Length + "\t";
                // }
                // else
                // {
                //     result += contents.Length + "\t";
                // }

                // if (!string.IsNullOrEmpty(file))
                // {
                //         FileInfo fileInfo = new FileInfo(file);
                //         result += fileInfo.Length + "\t"; // 输出文件大小
                // }

                // long fileSize=0;
                // if(!string.IsNullOrEmpty(contents)) {
                //     fileInfo=new FileInfo(file);
                //     fileSize=fileInfo.Length;
                // }
                // result+=fileSize+"\t"; // 文件大小使用Length属性
                result+=contents.Length+"\t";
                break;
            }
            case 'l':
                // C# 中没有直接计算行数的方法，这里简化处理，实际应用中可能需要更复杂的逻辑
                result += contents.Split('\n').Length - (contents.EndsWith("\n") ? 1 : 0)+ "\t"; 
                break;
            case 'w':
                result += contents.Split(new char[] { ' ', '\t', '\n', '\r' }, StringSplitOptions.RemoveEmptyEntries).Length + "\t"; // 分割并去除空白项计数
                break;
            case 'm': 
                // contents = File.ReadAllText(file, new UTF8Encoding(false));
                // result += contents.Length + "\t";
                // result+=contents.Length + "\t"; // 输出文件大小

                // using (StreamReader reader = new StreamReader(file))
                // {
                //     int characterCount = 0;
                //     int currentChar;

                //     // 逐字符读取文件
                //     while ((currentChar = reader.Read()) != -1)
                //     {
                //         characterCount++;
                //     }

                //     // 输出字符数
                //     // Console.WriteLine("Number of characters in the file: " + characterCount);
                //     result += characterCount + "\t"; // 输出文件大小
                // }

                result+=contents.Length + "\t"; // 输出文件大小

                break;
            default:
                // 处理未知命令字符
                result += $"Unknown command '{ch}'\t";
                break;
            }
        }
        if(should_output){
            return result+(string.IsNullOrEmpty(file) ? "" : file);
        } else {
            return "";
        }
        
    }
}



// douxiaobo@192 CSharp % dotnet new console --framework net8.0 --use-program-main
// 已成功创建模板“控制台应用”。

// 正在处理创建后操作...
// 正在还原 /Users/douxiaobo/Documents/Coding/Practice in Coding/build_your_own_wc_tool/CSharp/CSharp.csproj:
//   Determining projects to restore...
//   Restored /Users/douxiaobo/Documents/Coding/Practice in Coding/build_your_own_wc_tool/CSharp/CSharp.csproj (in 69 ms).
// 已成功还原。

// douxiaobo@192 CSharp % dotnet build                           
// 适用于 .NET MSBuild 版本 17.8.5+b5265ef37
//   Determining projects to restore...
//   All projects are up-to-date for restore.
//   CSharp -> /Users/douxiaobo/Documents/Coding/Practice in Coding/build_your_own_wc_tool/CSharp/bin/Debug/net8.0/CSharp.dll

// 已成功生成。
//     0 个警告
//     0 个错误

// 已用时间 00:00:00.46
// douxiaobo@192 CSharp % ./bin/Debug/net8.0/CSharp ./test.txt -c
// Command: c, File: ./test.txt
// 	342190	./test.txt
// douxiaobo@192 CSharp % ./bin/Debug/net8.0/CSharp ./test.txt -l
// Command: l, File: ./test.txt
// 	7145	./test.txt
// douxiaobo@192 CSharp % ./bin/Debug/net8.0/CSharp ./test.txt -w
// Command: w, File: ./test.txt
// 	58164	./test.txt



// douxiaobo@192 CSharp % dotnet build
// 适用于 .NET MSBuild 版本 17.8.5+b5265ef37
//   Determining projects to restore...
//   All projects are up-to-date for restore.
//   CSharp -> /Users/douxiaobo/Documents/Coding/Practice in Coding/build_your_own_wc_tool/CSharp/bin/Debug/net8.0/CSharp.dll

// 已成功生成。
//     0 个警告
//     0 个错误

// 已用时间 00:00:00.43
// douxiaobo@192 CSharp % ./bin/Debug/net8.0/CSharp ./test.txt -m      错
// Command: m, File: ./test.txt
// 	339291	./test.txt

// douxiaobo@192 build_your_own_wc_tool % cat test.txt | ./bin/Debug/net8.0/CSharp -c
// zsh: no such file or directory: ./bin/Debug/net8.0/CSharp
// cat: test.txt: No such file or directory
// douxiaobo@192 build_your_own_wc_tool % 


// douxiaobo@192 CSharp % cat test.txt | ./bin/Debug/net8.0/CSharp -w     对
// Command: w, File: 
// 	58164	
// douxiaobo@192 CSharp % cat test.txt | ./bin/Debug/net8.0/CSharp -c       错
// Command: c, File: 
// 	332147	
// douxiaobo@192 CSharp % cat test.txt | ./bin/Debug/net8.0/CSharp -l       对
// Command: l, File: 
// 	7145	
// douxiaobo@192 CSharp % cat test.txt | ./bin/Debug/net8.0/CSharp -m       错
// Command: m, File: 
// 	332147	
// douxiaobo@192 CSharp % 