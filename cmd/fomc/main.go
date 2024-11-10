package main

import (
	"encoding/xml"
	"fmt"
	"io"
	"log"
	"net/http"
	"strings"

	utils "github.com/mmclinton/fomc-rss/pkgs"
)

const fomcLink string = "https://www.federalreserve.gov/feeds/videos.xml"

type RSS struct {
	Channel Channel `xml:"channel"`
}

type Channel struct {
	Title string  `xml:"title"`
	Items []Video `xml:"item"`
}

type Video struct {
	Title   string `xml:"title"`
	Link    string `xml:"link"`
	PubDate string `xml:"pubDate"`
}

func main() {
	resp, err := http.Get(fomcLink)
	if err != nil {
		log.Fatalln(err)
	}
	defer resp.Body.Close()

	data, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("Error reading data: %v", err)
	}

	var rss RSS
	if err := xml.Unmarshal(data, &rss); err != nil {
		log.Fatalf("Error unmarshalling XML: %v", err)
	}

	fmt.Println("Channel Title:", rss.Channel.Title)
	fmt.Println()

	for _, item := range rss.Channel.Items {
		if strings.Contains(item.Title, "FOMC Press Conference") {
			fmt.Println(utils.CyanColor+"Item Title:"+utils.ResetColor, item.Title, utils.ResetColor)
			fmt.Println(utils.CyanColor+"Date:"+utils.ResetColor, item.PubDate)
			fmt.Println(utils.CyanColor+"Link:"+utils.ResetColor, item.Link)
			fmt.Println()
		}
	}
}
