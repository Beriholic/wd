package cmd

import (
	"fmt"
	"os"

	"github.com/Beriholic/wd/repo"
	"github.com/Beriholic/wd/ui"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "wd",
	Short: " A tui program for Chinese and English offline dictionaries in the terminal ",
	Run: func(cmd *cobra.Command, args []string) {
		repo.InitDB()

		p := tea.NewProgram(
			ui.InitMainUIViewModel(),
			tea.WithAltScreen(),
		)

		if _, err := p.Run(); err != nil {
			fmt.Println("Error running program:", err)
			os.Exit(1)
		}
	},
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
