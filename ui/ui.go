package ui

import (
	"strings"

	"github.com/beriholic/wd/repo"
	"github.com/beriholic/wd/repo/model"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

type MainUIViewModel struct {
	screenWidth     int
	screenHeight    int
	wordInput       textinput.Model
	wordText        string
	translationText string
	definitionText  string
	tagText         string
	exchangeText    string
}

func InitMainUIViewModel() MainUIViewModel {

	return MainUIViewModel{
		wordInput:       NewWordInput(),
		wordText:        "",
		translationText: "",
		definitionText:  "",
		tagText:         "",
		exchangeText:    "",
	}
}

func (vm MainUIViewModel) Init() tea.Cmd {
	return textinput.Blink
}

func (vm MainUIViewModel) View() string {
	return vm.renderMainUI(
		vm.wordTextView(),
		vm.translationTextView(),
		vm.definitionTextView(),
		vm.exchangeTextView(),
		vm.wordInputView(),
	)
}

func (vm MainUIViewModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd

	switch msgWithType := msg.(type) {
	case tea.WindowSizeMsg:
		vm.screenWidth = msgWithType.Width - 4
		vm.screenHeight = msgWithType.Height
	case tea.KeyMsg:
		switch msgWithType.String() {
		case "i":
			vm.wordInput.Focus()
			return vm, nil
		case "enter":
			if vm.wordInput.Value() == "" {
				return vm, nil
			}
			word := vm.fetchWordInfo()
			if word == nil {
				vm.wordText = "单词未找到"
				return vm, nil
			}

			vm.wordText = word.Word
			vm.translationText = word.Translation
			vm.definitionText = word.Definition
			vm.tagText = word.Tag
			vm.exchangeText = word.Exchange
			vm.wordInput.SetValue("")

			return vm, nil
		case "ctrl+c":
			return vm, tea.Quit
		case "q":
			if vm.wordInput.Focused() {
				return vm, nil
			}
			return vm, tea.Quit
		case "esc":
			if !vm.wordInput.Focused() {
				return vm, nil
			}
			vm.wordInput.Blur()
			return vm, nil
		}
	}

	vm.wordInput, cmd = vm.wordInput.Update(msg)

	return vm, cmd
}

func (vm MainUIViewModel) wordTextView() string {
	b := NewDefaultBoxWithLabel()

	return b.Render("Word", vm.wordText, vm.screenWidth, 0)
}

func (vm MainUIViewModel) translationTextView() string {
	b := NewDefaultBoxWithLabel()

	return b.Render("Translation", vm.translationText, vm.screenWidth, vm.screenHeight/4)
}

func (vm MainUIViewModel) definitionTextView() string {
	b := NewDefaultBoxWithLabel()
	return b.Render("Definition", vm.definitionText, vm.screenWidth, vm.screenHeight/4)
}

func (vm MainUIViewModel) wordInputView() string {
	b := NewDefaultBoxWithLabel()

	return b.Render("Input", vm.wordInput.View(), vm.screenWidth, 0)
}

func (vm MainUIViewModel) exchangeTextView() string {
	exchanges := strings.Split(vm.exchangeText, "/")

	exchangesText := strings.Builder{}

	for _, e := range exchanges {
		if len(e) < 2 {
			continue
		}
		wordType := e[:1]
		word := e[2:]

		switch wordType {
		case "p":
			exchangesText.WriteString("过去式：" + word)
		case "d":
			exchangesText.WriteString("过去分词：" + word)
		case "i":
			exchangesText.WriteString("现在分词：" + word)
		case "3":
			exchangesText.WriteString("第三人称单数：" + word)
		case "r":
			exchangesText.WriteString("形容词比较级：" + word)
		case "t":
			exchangesText.WriteString("形容词最高级：" + word)
		case "s":
			exchangesText.WriteString("名词复数形式：" + word)
		case "0":
			exchangesText.WriteString("Lemma：" + word)
		case "1":
			exchangesText.WriteString("Lemma变换形式：" + word)
		default:
			exchangesText.WriteString("未知：" + word)
		}
		exchangesText.WriteString("\n")
	}

	b := NewDefaultBoxWithLabel()

	return b.Render("Exchange", exchangesText.String(), vm.screenWidth, vm.screenHeight/5)
}

func (vm MainUIViewModel) footView() string {
	return lipgloss.NewStyle().Bold(true).Render(
		"Press 'i' to input a word | Press 'q' to quit",
	)

}

func (vm MainUIViewModel) renderMainUI(comps ...string) string {
	ui := strings.Builder{}

	for _, c := range comps {
		ui.WriteString(c)
		ui.WriteString("\n")
	}

	ui.WriteString("\n\n")
	ui.WriteString(vm.footView())

	return lipgloss.NewStyle().
		Height(vm.screenHeight).
		Render(ui.String())
}

func (vm MainUIViewModel) fetchWordInfo() *model.Stardict {
	word, err := repo.QueryWord(vm.wordInput.Value())

	if err != nil {
		return nil
	}

	return word
}
