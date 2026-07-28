use std::{collections::HashMap, todo};
use serde_json::Value as Json;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::db::class::class::Class;



#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ClassInstance {
    pub class_instance_id: Uuid,
    pub class_id: Uuid,
    pub parent: Option<Uuid>
}


// ASSOCIATED

#[derive(Clone, Debug, FromRow)]
pub struct ClassClassInstance {
    pub class_id: Uuid,
    pub name: String,
    pub fields: Json,
    pub schema: Json,
    pub class_instance_id: Uuid
}

impl Into<Class> for ClassClassInstance {
    fn into(self) -> Class {
        Class { class_id: self.class_id, name: self.name, fields: self.fields, schema: self.schema }
    }
}

// FROM TABLE TREE OBJECT

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ClassInstanceTreeLeaf {
    pub class_instance_id: Uuid,
    pub name: String,
    pub parent: Option<Uuid>
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClassInstanceTree {
    pub class_instance_id: Uuid,
    pub name: String,
    pub children: Vec<ClassInstanceTree>
}

impl ClassInstanceTree {

    pub fn to_tree(vec: Vec<ClassInstanceTreeLeaf>) -> Vec<Self> {

        let mut out: Vec<ClassInstanceTree> = Vec::new();

        let mut nodes: HashMap<Uuid, ClassInstanceTreeLeaf> = HashMap::new();
        let mut children: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        let mut roots: Vec<Uuid> = Vec::new();

        for c in vec {

            match c.parent {
                Some(parent) => {
                    children
                        .entry(parent)
                        .or_default()
                        .push(c.class_instance_id);
                },
                None => {
                    roots.push(c.class_instance_id)
                },
            }

            nodes.insert(c.class_instance_id, c);
            
        }

        for root in roots {
            out.push(
                build_tree(root, &mut nodes, &children)
            );
        }


        return out;
    }
}

fn build_tree(
    id: Uuid,
    nodes: &mut HashMap<Uuid, ClassInstanceTreeLeaf>,
    children: &HashMap<Uuid, Vec<Uuid>>,
) -> ClassInstanceTree {
    let node = nodes.remove(&id).expect("missing node");

    let children = children
        .get(&id)
        .into_iter()
        .flatten()
        .map(|child| build_tree(*child, nodes, children))
        .collect();

    ClassInstanceTree {
        class_instance_id: node.class_instance_id,
        name: node.name,
        children
    }
}