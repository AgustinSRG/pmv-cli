// Script to generate MANUAL.md

"use strict";

const Path = require("path");
const FS = require("fs");
const ChildProcess = require("child_process");

const PMV_CLI_BIN = Path.resolve(__dirname, "target", "release", "pmv-cli");
const MAN_MD_FILE = Path.resolve(__dirname, "MANUAL.md");

async function callPMV_CLI(args) {
    return new Promise((resolve, reject) => {
        try {
            const p = ChildProcess.spawn(PMV_CLI_BIN, args);

            let result = "";

            p.stdout.on("data", data => {
                result += data.toString();
            });

            p.stdout.on("end", () => {
                resolve(result);
            });
        } catch (e) {
            reject(e);
        }
    });
}

function writeResult(res) {
    FS.writeFileSync(MAN_MD_FILE, res);
}

function parseHelp(text) {
    const lines = text.split("\n");

    const commandDesc = lines[0] || "";

    let usage = "";
    let commands = [];
    let args = [];
    let options = [];

    let current = "";
    
    for (let i = 1; i < lines.length; i++) {
        let line = lines[i].trim();

        if (!line) {
            current = "";
        }

        if (line.startsWith("Usage:") && !current) {
            usage = line.split(":").slice(1).join(":").trim();
            continue;
        } else if (line === "Commands:") {
            current = "c";
            continue;
        } else if (line === "Options:") {
            current = "o";
            continue;
        } else if (line === "Arguments:") {
            current = "a";
            continue;
        }

        const spl = line.split("  ");

        if (current === "c") {
            // Command
            const commandName = spl[0].trim();
            const commandDesc = spl.slice(1).join("  ").trim();

            if (commandName !== "help") {
                commands.push({
                    name: commandName,
                    desc: commandDesc,
                });
            }
        } else if (current === "o") {
            // Option
            const optionSyntax = spl[0].trim();
            const optionDesc = spl.slice(1).join("  ").trim();

            options.push({
                syntax: optionSyntax,
                desc: optionDesc,
            });
        } else if (current === "a") {
            // Option
            const argName = spl[0].trim();
            const argDesc = spl.slice(1).join("  ").trim();

            args.push({
                name: argName,
                desc: argDesc,
            });
        }
    }

    return {
        desc: commandDesc,
        usage: usage,
        args: args,
        commands: commands,
        options: options,
    };
}

function toMd(ph, cmdStack) {
    const baseLink = "#command" + (cmdStack.length > 0 ? ("-" + cmdStack.join("-")) : "");
    const lines = [];

    lines.push(ph.desc);
    lines.push("");

    lines.push("<ins>**Usage:**</ins>");
    lines.push("");
    lines.push("```");
    lines.push(ph.usage)
    lines.push("```");
    lines.push("");

    if (ph.commands.length > 0) {
        lines.push("<ins>**Commands:**</ins>");
        lines.push("");
        lines.push("| Command | Description |");
        lines.push("| --- | --- |");

        for (let cmd of ph.commands) {
            lines.push(`| [${cmd.name}](${baseLink}-${cmd.name}) | ${cmd.desc} |`);
        }

        lines.push("");
    }

    if (ph.args.length > 0) {
        lines.push("<ins>**Arguments:**</ins>");
        lines.push("");
        lines.push("| Argument | Description |");
        lines.push("| --- | --- |");

        for (let arg of ph.args) {
            lines.push(`| \`${arg.name}\` | ${arg.desc} |`);
        }

        lines.push("");
    }

    if (ph.options.length > 0) {
        lines.push("<ins>**Options:**</ins>");
        lines.push("");
        lines.push("| Option | Description |");
        lines.push("| --- | --- |");

        for (let opt of ph.options) {
            lines.push(`| \`${opt.syntax}\` | ${opt.desc} |`);
        }

        lines.push("");
    }

    return lines;
}

async function resolveRecursive(level, lines, cmdStack) {
    let prefixTitle = "#";

    for (let i = 0; i < level; i++) {
        prefixTitle += "#";
    }

    if (cmdStack.length > 0) {
        lines.push(prefixTitle + " Command: " + cmdStack.join(" "));
    } else {
        lines.push(prefixTitle + " Manual");
    }

    lines.push("");

    const baseHelp = parseHelp(await callPMV_CLI(cmdStack.concat(["--help"])));

    toMd(baseHelp, cmdStack).forEach(line => {
        lines.push(line);
    });

    for (let cmd of baseHelp.commands) {
        if (cmd.name === "help") {
            continue;
        }

        await resolveRecursive(level + 1, lines, cmdStack.concat(cmd.name));
    }
}

async function main() {
    const lines = [];

    await resolveRecursive(0, lines, []);

    writeResult(lines.join("\n"));
}

main().catch(e => {
    console.error(e);
    process.exit(1);
});


