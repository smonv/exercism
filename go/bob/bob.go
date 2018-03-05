// Package bob implement Bob's response
package bob

import (
	"strings"
)

// Hey return response of Bob
func Hey(remark string) string {
	remark = strings.TrimSpace(remark)
	if remark == "" {
		return "Fine. Be that way!"
	}
	if string(remark[len(remark)-1]) == "?" {
		if strings.ToUpper(remark) == remark && strings.ToLower(remark) != remark {
			return "Calm down, I know what I'm doing!"
		}
		return "Sure."
	}
	if strings.ToUpper(remark) == remark && strings.ToLower(remark) != remark {
		return "Whoa, chill out!"
	}

	return "Whatever."
}
