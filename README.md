# WIP. I haven't even proofread things. 
 
# Known Knowledge

A web application built for collaborative knowledge archiving and transmission. This proposal hopes to accomplish at least the following

* Knowledge from any educational material can interface with eachother.
	* e.g. determine the overlap between your high school biology class and a college level one. 

* Easily define your own knowledge tree 

* Knowledge is always attached with study materials

* Map out the order for which prerequisites of different skills and professions could be learned.

* Establish a marketplace for lessons, exercises, and tests so we can stop rewriting those and use the most effective material (or maybe the most liked).

* Track users’ learning to fill knowledge gaps

* Track user’s learning to point out when a concept hasn’t been engaged with for a while and may go stale. 



## How is knowledge stored?

### `idea`, `group`, and `knowledge tree`

In this application, all of knowledge has two categorizations.

* An `idea` is a self-contained knowable thing.
* A `group` is the connective tissue that explains how `idea`s and/or `group`s relate.

From this point forward, a collection of `idea`s and `group`s is called a `knowledge tree`.

#### Elaboration on `idea`

An idea is composed of 4 part, all for identification purposes, crazy enough.

```Rust
pub struct Idea {
	id: uuid,
	name: String,
	context: Option<String>,
	description: Option<String>,
}
```

The name is something to call the idea. It has a context in case there are two ideas with the same name but exist in different domains. There’s a description/definition to quickly help someone determine if this idea is useful. Lastly the id, a uuid, is a random 128 bit label. There are $2^122$ possible uuids meaning they’ll always be unique without need for cross validation. This will have implementation value down the road when collaborating. 

##### Additional `idea` Information 

There are additional things that will be stored to help get the idea in a person’s brain.

Each `idea` may have the following community created add-ons (they’ll just be links to other websites at the start).
* `Explanations` that seek to explain the idea fully
* `Exercises` a bunch of problems that help teach the concept, and make it less theoretical.
* `Tests` in some cases people may submit tests that measure the efficacy of the `Explanations` and `Exercises` 





### Elaboration on `group`

A `group` is a structure that connects the `idea`s or other `group`s. Whereas ideas are meant to be used in many different contexts, `group`s can be built around a specific purpose, like a Web Developer Basics `group`. At the moment, there are three types of `group`s.

* list, a bin of related ideas
* numbered list, a list of ideas to be understood in a certain order
* DAG, directed acyclic graph, a more general form 


```Rust
// not actually the planned implementation
pub struct Group {
	id: uuid,
	name: String,
	description: Option<String>,
	associated_idea: Option<Idea>,
	type: GraphType,
	payload: Data	,
}
```

### Groups and Ideas in action

If I want to store what I just covered in a Knowledge Tree, I have a lot of ways of going about it. The simplest thing I could do would be to just put it all in a single idea called Known Knowledge Proposal. The text here would be the `explanation` and it could be possibly one of many. If this was something I expected people to learn, I could provide a few `exercise`s like notecards or interactive demos.

However, we could make a more modular structure starting with the `group` Known Knowledge Proposal.

Known Knowledge Proposal (Group, Numbered List)
1. UUID
2. idea (idea)
3. group (idea)
			
In this contrived example, by adding a little structure we can extract the uuid information. We can show that both `idea` and `group` rely on the concept of uuid. We can even pull an existing uuid idea, so we don’t have to re-explain it. Also once the user reads through it they maybe able to apply that knowledge elsewhere like if they came across the Programming a Todo List `group` or maybe even a data engineer certification `group`. 

## Mockup

## TODO

1. Create overarching proposal
2. Add very basic UI (UI will be built in conjunction with BE)
3. Add basic grouping of ideas
4. ...
