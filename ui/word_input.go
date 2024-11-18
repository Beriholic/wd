package ui

import (
	"github.com/charmbracelet/bubbles/textinput"
)

func NewWordInput() textinput.Model {
	input := textinput.New()
	input.Placeholder = "Enter a word"
	input.CharLimit = 20
	return input
}
