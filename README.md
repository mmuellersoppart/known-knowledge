# WIP. I haven't even proofread things. 
 
# Known Knowledge

A web application built for collaborative knowledge archiving and transmission. This proposal hopes to accomplish at least the following

* All educational material can interface with eachother.
	* e.g. determine the overlap between your high school biology class and a college level one. 

* Easily store your knowledge and take ownership over how it's structured based off your interests. 

* Create the expectation that all knowledge should come with study materials.

* Map out the prerequisites for skills and even professions.

* Establish a marketplace for lessons, exercises, and tests so we can stop rewriting those and let the best material emerge.

* Track users’ learning for tailored coaching using techniques like spaced repetition.


## How is knowledge stored?

### `idea`, `group`, and `knowledge tree`

In this application, all of knowledge has two categorizations.

* An idea is a self-contained knowable thing.
* A group is the connective tissue that explains how ideas and/or groups relate.

From this point forward, a collection of ideas and groups is called a `knowledge tree`.

#### Elaboration on idea

An idea is composed of 4 part, all for identification purposes, crazy enough.

```Rust
pub struct Idea {
	id: uuid,
	name: String,
	context: Option<String>,
	description: Option<String>,
}
```

The name is something to call the idea. In case there is a name colision (two of the same names) a context is provided as well. There’s a description/definition to quickly help someone determine if this idea is useful. Lastly the id, a uuid, is a random 128 bit label. There are $2^122$ possible uuids meaning they’ll always be unique without need for cross validation. This will important for collaboration. 

### Elaboration on group

A group is a structure that connects the ideas and/or other groups. At the moment, there are three types of groups.

* list, a bin of related ideas
* numbered list, a list of ideas to be understood in a certain order
* DAG, directed acyclic graph, a more general form of ordered list than the numbered list

```Rust
// not the actual implementation
pub struct Group {
	id: uuid,
	name: String,
	description: Option<String>,
	associated_idea: Option<Idea>,
	type: GraphType,
	payload: Data	,
}
```


##### Additional idea or group information 

Additional information can be added to ideas or groups that help help get the information in a person’s brain.

Here are the possible community created add-ons (they’ll just be links to other websites at the start).
* `Explanations` that seek to explain the idea fully.
* `Exercises` are a bunch of problems that help teach the concept.
* `Tests` maybe submitted so that we can measure the efficacy of the explanations and exercises. 

### Groups and Ideas in action

If I want to store what I just covered in a Knowledge Tree, I have a lot of ways of going about it. The simplest thing I could do would be to just put it all in a single idea called Known Knowledge Proposal. The text here would be the explanation and it could be possibly one of many. If this was something I expected people to learn, I could provide a few exercises like notecards or an interactive demo.

However, we could make a more modular structure starting with the group Known Knowledge Proposal.

Known Knowledge Proposal (Group, Numbered List)
1. UUID
2. idea (idea)
3. group (idea)
			
In this contrived example, by adding a little structure we can extract the uuid information. We can show that both idea and group rely on the concept of uuid. We can even pull an existing uuid idea, so we don’t have to re-explain it. Also once the user reads through the uuid description and exercises, they maybe able to apply that knowledge elsewhere like if they came across the Programming a Todo List `group` or maybe it could even go towards a data engineer certification `group`. 

## Mockup






## What is knowledge? 

Knowledge is information our brains can store. We can obtain information with our senses and, upon reflection, sometimes, we can derive new conclusions from what we know. With the totality of experience, we attempt to understand reality, so we can try to operate in this world. 

Here’s what oxford has to say.

> Facts, information, and skills acquired through experience or education; the theoretical or practical understanding of a subject:

Pretty close! 

The definitions do not capture the dizzying scale of knowledge. First, there is an unlimited amount of facts, information, and skills. To make things worse, each fact and piece information can interact with any other fact or piece of information. As a person with limitations, we have to constantly decide what is and isn’t important. More so,

 Knowledge is hard work. All the knowledge in existence currently exists in our collective heads. If all people were to die, then we’d lose all our knowledge despite books still existing.  
 
## Mockup

## TODO

1. Create overarching proposal
2. Add very basic UI (UI will be built in conjunction with BE)
3. Add basic grouping of ideas
4. ...
