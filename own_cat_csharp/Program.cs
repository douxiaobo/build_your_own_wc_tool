namespace own_cat_csharp;
using System;  
using System.IO;  

class Program
{
    static void Main(string[] args)
    {
        // Console.WriteLine("Hello, World!");
        string filePath = "test_file";  

        if (string.IsNullOrEmpty(filePath))
        {
            Console.WriteLine("文件路径不能为空");
            return;
        }

        filePath = Path.Combine(Directory.GetCurrentDirectory(), filePath);
  
        try  
        {              
            // 使用StreamReader来读取文件  
            using (StreamReader sr = new StreamReader(filePath))  
            {  
                string line;  
                // 逐行读取文件并打印到控制台  
                while ((line = sr.ReadLine()) != null)  
                {  
                    Console.WriteLine(line);  
                }  
            }  
        }  
        catch (Exception e)  
        {  
            // 处理可能出现的异常，例如文件未找到  
            Console.WriteLine("读取文件时出错: " + e.Message);  
        }  
    }
}

// Last login: Fri Jun  7 21:02:36 on ttys003
// douxiaobo@192 own_cat_csharp % dotnet new console --framework net8.0 --use-program-main
// 已成功创建模板“控制台应用”。

// 正在处理创建后操作...
// 正在还原 /Users/douxiaobo/Documents/Coding/Practice in Coding/build_your_own_wc_tool/own_cat_csharp/own_cat_csharp.csproj:
//   Determining projects to restore...
//   Restored /Users/douxiaobo/Documents/Coding/Practice in Coding/build_your_own_wc_tool/own_cat_csharp/own_cat_csharp.csproj (in 1.69 sec).
// 已成功还原。


// douxiaobo@192 own_cat_csharp % code .
// douxiaobo@192 own_cat_csharp % dotnet run
// !
// 1
// 12
// 345
// 67890
// douxiaobo@192 own_cat_csharp % 

