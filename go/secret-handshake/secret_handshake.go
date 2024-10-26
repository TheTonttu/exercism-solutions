package secret

import "slices"

type HandshakeAction uint

const (
	Wink         HandshakeAction = 0b00001
	DoubleBlink  HandshakeAction = 0b00010
	CloseEyes    HandshakeAction = 0b00100
	Jump         HandshakeAction = 0b01000
	ReverseOrder HandshakeAction = 0b10000
)

var actions = []HandshakeAction{
	Wink, DoubleBlink, CloseEyes, Jump,
}

func (action HandshakeAction) String() string {
	switch action {
	case Wink:
		return "wink"
	case DoubleBlink:
		return "double blink"
	case CloseEyes:
		return "close your eyes"
	case Jump:
		return "jump"
	case ReverseOrder:
		return "reverse order"
	}
	return ""
}

func Handshake(code uint) []string {
	handshake := []string{}

	for _, action := range actions {
		if code&uint(action) != 0 {
			handshake = append(handshake, action.String())
		}
	}

	if code&uint(ReverseOrder) != 0 {
		slices.Reverse(handshake)
	}

	return handshake
}
