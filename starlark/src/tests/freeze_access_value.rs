/*
 * Copyright 2018 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use anyhow::Context as _;

use crate::values::{list::List, Freeze, Freezer, FrozenHeap, FrozenValue, Heap, Value};

struct Test<V> {
    field: V,
}

impl<'v> Freeze for Test<Value<'v>> {
    type Frozen = Test<FrozenValue>;

    fn freeze(self, freezer: &Freezer) -> anyhow::Result<Self::Frozen> {
        let test = Test {
            field: self.field.freeze(freezer)?,
        };
        let _ignored = List::from_value(test.field.to_value()).context("Not a list!")?;
        Ok(test)
    }
}

#[test]
fn test() -> anyhow::Result<()> {
    let heap = Heap::new();
    let list = heap.alloc(vec![1i32, 2i32]);

    let t = Test { field: list };

    let freezer = Freezer::new(FrozenHeap::new());
    list.freeze(&freezer)?;
    t.freeze(&freezer)?;

    Ok(())
}
