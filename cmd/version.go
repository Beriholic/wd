package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

const version = "0.3.0"

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Print the version number of wd",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("wd version %s\n", version)
	},
}

func init() {
	rootCmd.AddCommand(versionCmd)
}
