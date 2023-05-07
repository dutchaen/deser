# deser
json to deserializable structure program (supports C#, Rust, and Go)

## what does this do
this program will convert your json string into deserializable structures so you access json data easily in your code

## ok so why is this awesome
because some json strings are long and it would be a pain to do the same thing by hand so why not use a program to do it? you could technically use an object which could represent the json like JObject (C#), map[string]interface{} (Go) or serde_json::Value (Rust) but i think these solutions look bad in code and you have to do things like casting the object into the correct type or manually handling Option types (or unwrapping which is way worse) which is unsafe and ugly imo

### super not cool example:
 ```
package main

import (
	"encoding/json"
	"fmt"
)

func main() {
	item := []byte(`{
      "ancient": "glass",
      "package": true,
      "milk": {
        "goes": [
          "no",
          "where",
          "here"
        ],
        "also": "clear",
        "station": "yard",
        "corner": "specific",
        "event": true,
        "blanket": true
      },
      "give": -1409844251,
      "fallen": "high",
      "coffee": 2125499180
    }`)

	var json_object map[string]interface{}
	if err := json.Unmarshal(item, &json_object); err != nil {
		panic(err)
	}
    
    // you see why i think that this is ugly? constant casting... long line of code.. uses runtime type checks constantly (super bad) (should have been done on line 40-43)
	where_milk := json_object["milk"].(map[string]interface{})["goes"].([]interface{})
    // ^^ super duper stinky code

	location := ""
	for _, where := range where_milk {
		location += where.(string)
	}
	fmt.Println(location)
}
 ```
 
 ### super awesome example:
 ```
package main

import (
   "encoding/json"
   "fmt"
)

// amazing structures created by this program!!!
type WeirdItem struct {
	Ancient string  `json:"ancient"`
	Coffee  float64 `json:"coffee"`
	Fallen  string  `json:"fallen"`
	Give    float64 `json:"give"`
	Milk    Milk    `json:"milk"`
	Package bool    `json:"package"`
}

type Milk struct {
	Also    string   `json:"also"`
	Blanket bool     `json:"blanket"`
	Corner  string   `json:"corner"`
	Event   bool     `json:"event"`
	Goes    []string `json:"goes"`
	Station string   `json:"station"`
}

func main() {
    item := []byte(`{
      "ancient": "glass",
      "package": true,
      "milk": {
        "goes": [
          "no",
          "where",
          "here"
        ],
        "also": "clear",
        "station": "yard",
        "corner": "specific",
        "event": true,
        "blanket": true
      },
      "give": -1409844251,
      "fallen": "high",
      "coffee": 2125499180
    }`)

	var weird_item WeirdItem
	if err := json.Unmarshal(item, &weird_item); err != nil {
		panic(err)
	}
    
    // THIS IS PEAK EVOLUTION! THIS IS IT BABY YEAH! YEAH BABY YAHOOOOOOOOO!!!!!!!!!!!!!!
    location := ""
    // ------------------------\   LOOK AT IT LOOK AT IT!!!!!
    for _, where := weird_item.Milk.Goes {
        location += where
        //^^ YES YES YES  YES YES YES YES!!!!
    }
    fmt.Println(location)
}
```

## so how do i use this amazing program??
just copy the json string to your clipboard, run the application from the release section, pick your target language via index and name the initial structure then voila!! ez pz structures!!!!

## any downsides
i dont think so... maybe... i dont know... not really.. dont be too mean with it >:(
