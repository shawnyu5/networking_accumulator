package middware

import (
	"log"
	"time"

	"github.com/bwmarrin/discordgo"
	"github.com/shawnyu5/networking_bot/commands"
)

// Slash command handler logger
type Logger struct {
	Logger *log.Logger
	Next   commands.CommandStruct
}

// handler calls discord slash command handler with logging
func (l Logger) Handler(sess *discordgo.Session, i *discordgo.InteractionCreate) (string, error) {
	output, err := l.Next.Handler(sess, i)
	defer func(begin time.Time, output string) {
		l.Logger.Printf("command=%s response='%s' err=%s took=%s", l.Next.Name, output, err, time.Since(begin))
	}(time.Now(), output)

	return output, err
}

// NewLogger creates a new logger middware.
// logger: logger to use.
// next: next middware in chain.
func NewLogger(logger *log.Logger, next commands.CommandStruct) Logger {
	return Logger{Logger: logger, Next: next}
}
