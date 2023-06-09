/*
 *   Copyright (c) 2023 Duart Snel
 *   All rights reserved.

 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at

 *   http://www.apache.org/licenses/LICENSE-2.0

 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

type osadlObj= {
    licenses: {
        name: string,
        compatibilities: {
            name: string,
            compatibility: "Same" | "Yes" | "No" | "Unknown",
            explanation: string
        }[]
    }[],
}

const separator = "[:::]"

function serializeEntries(entries: (string | number | boolean)[]) {
    return entries.join(separator)
}

fetch('https://www.osadl.org/fileadmin/checklists/matrixseqexpl.json').then(r => r.json()).then((res: osadlObj) => {
    let all_compatibilities = new Set<string>();
    let x = res.licenses.flatMap((leading) => {
        return leading.compatibilities.flatMap((subordinate) => {
            all_compatibilities.add(subordinate.compatibility)
            return serializeEntries([
                leading.name,
                subordinate.name,
                subordinate.compatibility
            ])
        })
    })
    .join('\n')

    Deno.writeTextFileSync("x.txt", x);
});