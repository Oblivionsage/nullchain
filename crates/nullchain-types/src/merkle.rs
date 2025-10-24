//! Merkle tree implementation for transaction commitment

use crate::hash::Hash256;
use serde::{Deserialize, Serialize};

/// Merkle tree for efficient transaction verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    /// Leaves (transaction hashes)
    leaves: Vec<Hash256>,
    /// Internal nodes
    nodes: Vec<Hash256>,
    /// Root hash
    root: Hash256,
}

impl MerkleTree {
    /// Build a Merkle tree from transaction hashes
    pub fn new(leaves: Vec<Hash256>) -> Self {
        if leaves.is_empty() {
            return Self {
                leaves: vec![],
                nodes: vec![],
                root: Hash256::zero(),
            };
        }
        
        let mut nodes = leaves.clone();
        let mut current_level = leaves.clone();
        
        // Build tree bottom-up
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for i in (0..current_level.len()).step_by(2) {
                let left = current_level[i];
                let right = if i + 1 < current_level.len() {
                    current_level[i + 1]
                } else {
                    // Duplicate last node if odd number
                    current_level[i]
                };
                
                let parent = Self::hash_pair(&left, &right);
                next_level.push(parent);
                nodes.push(parent);
            }
            
            current_level = next_level;
        }
        
        let root = current_level[0];
        
        Self {
            leaves,
            nodes,
            root,
        }
    }
    
    /// Get the Merkle root
    pub fn root(&self) -> Hash256 {
        self.root
    }
    
    /// Get number of leaves
    pub fn len(&self) -> usize {
        self.leaves.len()
    }
    
    /// Check if tree is empty
    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }
    
    /// Hash two nodes together (left || right)
    fn hash_pair(left: &Hash256, right: &Hash256) -> Hash256 {
        use blake3::Hasher;
        
        let mut hasher = Hasher::new();
        hasher.update(left.as_bytes());
        hasher.update(right.as_bytes());
        
        Hash256::from_bytes(*hasher.finalize().as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_tree() {
        let tree = MerkleTree::new(vec![]);
        assert_eq!(tree.root(), Hash256::zero());
        assert!(tree.is_empty());
    }
    
    #[test]
    fn test_single_leaf() {
        let leaf = Hash256::from_bytes([1u8; 32]);
        let tree = MerkleTree::new(vec![leaf]);
        assert_eq!(tree.root(), leaf);
        assert_eq!(tree.len(), 1);
    }
    
    #[test]
    fn test_two_leaves() {
        let leaf1 = Hash256::from_bytes([1u8; 32]);
        let leaf2 = Hash256::from_bytes([2u8; 32]);
        let tree = MerkleTree::new(vec![leaf1, leaf2]);
        
        // Root should be hash of both leaves
        assert_ne!(tree.root(), Hash256::zero());
        assert_eq!(tree.len(), 2);
    }
    
    #[test]
    fn test_deterministic() {
        let leaves = vec![
            Hash256::from_bytes([1u8; 32]),
            Hash256::from_bytes([2u8; 32]),
            Hash256::from_bytes([3u8; 32]),
        ];
        
        let tree1 = MerkleTree::new(leaves.clone());
        let tree2 = MerkleTree::new(leaves);
        
        assert_eq!(tree1.root(), tree2.root());
    }
}
