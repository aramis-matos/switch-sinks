package main

import (
	"os/exec"
	"strings"
)

func main() {
	cmd := exec.Command("bash", "-c", "pactl get-default-sink | xargs echo -n")
	out, _ := cmd.Output()
	curr := string(out[:])
	cmd = exec.Command("bash", "-c", "pactl list sinks | grep -E 'Name: .*' | sed 's/Name: //' | xargs echo -n")
	out, _ = cmd.Output()
	sources := strings.Split(string(out[:])," ")
	for index,source := range sources {
		if curr == source {
			cmd := exec.Command("pactl","set-default-sink",sources[(index +1) % len(sources)])
			cmd.Output()
			return
		}
	}
}
