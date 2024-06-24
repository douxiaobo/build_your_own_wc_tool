package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"unicode"
)

type File struct {
	FileName   string
	LineCount  uint32
	CharCount  uint32
	WordCount  uint32
	BytesCount uint32
}

func NewFile(filePath string, reader io.Reader) (*File, error) {
	lineCountTmp := uint32(0)
	charCountTmp := uint32(0)
	wordCountTmp := uint32(0)
	bytesCountTmp := uint32(0)
	currentWord := false

	bufReader := bufio.NewReader(reader)
	for {
		line, err := bufReader.ReadString('\n') // 读取直到换行符
		if err != nil && err != io.EOF {
			return nil, err
		}
		if err == io.EOF && len(line) == 0 {
			break
		}

		// lineStr := string(lineBytes)
		lineCountTmp++

		// for _, _ = range lineStr {
		// 	charCountTmp++ // 累积字符计数
		// }

		charCountTmp += uint32(len([]rune(line))) // 累积字符计数

		bytesCountTmp += uint32(len(line)) // 同上，关于+1的考虑

		for _, ch := range line {
			if unicode.IsSpace(ch) {
				if currentWord {
					wordCountTmp++
					currentWord = false
				}
			} else {
				currentWord = true
			}
		}
		if currentWord {
			wordCountTmp++
			currentWord = false
		}

	}

	return &File{
		FileName:   filePath,
		LineCount:  lineCountTmp,
		CharCount:  charCountTmp,
		WordCount:  wordCountTmp,
		BytesCount: bytesCountTmp,
	}, nil
}

func main() {
	args := os.Args
	if len(args) < 2 {
		fmt.Println("Usage: byowt <file>")
		return
	}
	// fmt.Println(args[1:])
	var command string
	var file_path string
	for _, arg := range args[1:] {
		// fmt.Println(arg)
		if strings.HasPrefix(arg, "-") {
			command += arg
		} else if is_file(arg) {
			file_path = arg
		} else {
			fmt.Fprintln(os.Stderr, "Invalid argument:", arg)
			os.Exit(1)
		}
	}
	// fmt.Printf("Command: %s File: %s\n", command, file_path)
	var reader io.Reader
	var err error
	if file_path == "" {
		// fmt.Fprintln(os.Stderr, "No file specified, reading from stdin...")
		reader = bufio.NewReader(os.Stdin)
	} else {
		reader, err = os.Open(filepath.Clean(file_path))
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error opening file %s: %v\n", file_path, err)
			os.Exit(1)
		}
		defer reader.(*os.File).Close()
	}

	file, err := NewFile(file_path, reader)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error creating file stats: %v\n", err)
		os.Exit(1)
	}

	// 现在您可以使用file.LineCount, file.WordCount等属性
	// fmt.Printf("File Name: %s\nLine Count: %d\nWord Count: %d\nCharacter Count: %d\nByte Count: %d\n",
	// 	file.FileName, file.LineCount, file.WordCount, file.CharCount, file.BytesCount)

	if command == "" {
		fmt.Printf("\t%d\t%d\t%d\t%s\n", file.LineCount, file.WordCount, file.BytesCount, file.FileName)
	} else {
		for _, ch := range command {
			if ch == rune('c') {
				fmt.Printf("\t%d", file.BytesCount)
			} else if ch == rune('l') {
				fmt.Printf("\t%d", file.LineCount) //正确的
			} else if ch == rune('w') {
				fmt.Printf("\t%d", file.WordCount) //正确的
			} else if ch == rune('m') {
				fmt.Printf("\t%d", file.CharCount)
			}
		}
		fmt.Println("\t" + file.FileName)
	}
}
func is_file(path string) bool {
	info, err := os.Stat(path)
	if err != nil {
		return false
	}
	return !info.IsDir()
}

// douxiaobo@192 Go % cat test.txt | go run byowt.go -c -l -w -m
// No file specified, reading from stdin...
// File Name:
// Line Count: 7145
// Word Count: 58164
// Character Count: 339292
// Byte Count: 342190
// 	342190	7145	58164	339292
// douxiaobo@192 Go % go mod init
// go: cannot determine module path for source directory /Users/douxiaobo/Documents/Practice in Coding/build_your_own_wc_tool/Go (outside GOPATH, module path must be specified)

// Example usage:
// 	'go mod init example.com/m' to initialize a v0 or v1 module
// 	'go mod init example.com/m/v2' to initialize a v2 module

// Run 'go help mod init' for more information.
// douxiaobo@192 Go % cat test.txt | go run byowt.go -c -l -w -m
// byowt.go:3:1: expected 'package', found 'import'
// douxiaobo@192 Go % go mod init byowt.local/tool
// go: creating new go.mod: module byowt.local/tool
// go: to add module requirements and sums:
// 	go mod tidy
// douxiaobo@192 Go % go mod tidy
// douxiaobo@192 Go %
