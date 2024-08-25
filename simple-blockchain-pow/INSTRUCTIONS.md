///////////////////////////////////////////////////////////////////////
// "mini-chain" SWE coding challenge in Scala, version 5
//
// COPYRIGHT © IOHK
///////////////////////////////////////////////////////////////////////
//
// We will be using this to have a technical discussion.
// A few focused hours should be enough but ultimately it is up to you.
// We definitely do not want you to spend many days of your personal
// time.
//
// Remember, this is a representation of your skill and ultimately the
// first point of contact with our technical recruitment team.
// If you think that something is worth doing - it's probably
// a good idea to show it off (eg. error handling, sensible unit test coverage,
formatting).
//
// You can think about this code as virtual collaboration with a
// colleague.
//
// General requirements are:
//
// - Your submission is an SBT project.
//
// - Your submission is a git repository with a commit log that you would submit
as a PR.
//
// - Ultimately it is up to the reviewer to understand the code submitted.
// They do their best but if the code is hard to follow
// that will affect the evaluation.
//
// - Review and complete the design and implementation.
//
// - The coding language is Scala.
//
// - You are free to come up with any solution you want but take into account
// memory and time complexity.
//
// - Comments stating "this could be done better with 'XYZ'" will be ignored.
// We can speculate all day but at the end the code submitted is what is
evaluated.
//
// - If you come up with a solution for a piece of the code,
// but it is inefficient/doesn't capture all scenarios
// and it would be very time consuming to implement a better one,
// please add a comment describing the improvement, in what way it is better
// and why you think it would be a lot of work.
//
// - You must devise some tests to cover the code
// that would support its maintenance over time and in a team setting.
// hint: this does not mean one test that shows one happy path engaging the
whole application.
//
// - Feel free to change any part of the design.
//
// - The code is written in an OO imperative style. We strongly encourage an FP
oriented solution.
//
// - You are free to add any external libraries you need.
//
// - You need to document the system requirements of the project (eg. major jvm
version)
// in a file called requirements.md.
//
// - You must share your submission privately (i.e. not a public GitHub
// repository). Typically, a zip archive with the local git repo project source
code.
//
// - Assume the code will run in a multi-threaded, multi-cpu environment.
// You never know how concurrency (or is it parallelism?) will kick in.
//
// - Whenever you are making changes to the supplied code base make a note with
a reason/purpose
//    (perhaps as part of the commit message).
//
///////////////////////////////////////////////////////////////////////
// We build some ingredients of a mini blockchain. Concepts we will
// be dealing with are:
//

//
//
// - Blockchain, a singly-linked list of Blocks with some extra
// API to build and query it.
//
// - Proof-of-Work or PoW for short.
// A computationally intensive algorithm whose role is to solve a
// puzzle that allows the next block to appear in the blockchain.
// We "mine" the next block by using PoW.
//
///////////////////////////////////////////////////////////////////////
