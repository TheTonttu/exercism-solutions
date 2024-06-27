package pangram

import "unicode"

func IsPangram(input string) bool {
	const Alphabets = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

	matches := make(map[rune]bool)
	for _, alphabet := range Alphabets {
		matches[alphabet] = false
	}

	for _, char := range input {
		upper := unicode.ToUpper(char)
		match, exists := matches[upper]
		if exists && !match {
			matches[upper] = true
		}
	}

	for _, match := range matches {
		if !match {
			return false
		}
	}
	return true
}
