package com.cpc.android

import com.cpc.social.models.*
import java.nio.ByteBuffer

object JniConversion {
    fun userToJni(user: User): Long {
        return NativeBridge.createUser(user.id, user.name, user.email)
    }

    fun postToJni(post: Post): Long {
        val commentRefs = post.comments.map { commentToJni(it) }.toLongArray()
        return NativeBridge.createPost(
            post.id, 
            post.content, 
            post.authorId, 
            post.likes,
            commentRefs,
            commentRefs.size
        )
    }

    fun commentToJni(comment: Comment): Long {
        return NativeBridge.createComment(comment.id, comment.content, comment.authorId)
    }

    fun proposalToJni(proposal: Proposal): Long {
        return NativeBridge.createProposal(
            proposal.id, 
            proposal.title, 
            proposal.description, 
            proposal.authorId
        )
    }

    fun feedItemToJni(feedItem: FeedItem): Long {
        return NativeBridge.createFeedItem(feedItem.id, feedItem.type, feedItem.content)
    }

    fun productToJni(product: Product): Long {
        return NativeBridge.createProduct(
            product.id,
            product.name,
            product.brand ?: "",
            product.description,
            product.barcode ?: "",
            product.carbonFootprint,
            product.packagingType ?: "",
            product.nutritionalInfo ?: "",
            product.manufacturer ?: "",
            product.materialCost,
            product.laborCost,
            product.supplier ?: "",
            product.currentStock,
            product.reorderLevel,
            product.supplyChain?.let { supplyChainToJni(it) } ?: 0,
            product.cost?.amount ?: 0.0,      // New cost field
            product.cost?.currency ?: "",      // New currency field
            product.location?.id ?: "",        // New location id
            product.location?.name ?: ""       // New location name
        )
    }

    fun fromJniUser(ref: Long): User {
        val id = NativeBridge.getUserId(ref)
        val name = NativeBridge.getUserName(ref)
        val email = NativeBridge.getUserEmail(ref)
        return User(id, name, email)
    }

    fun fromJniComment(ref: Long): Comment {
        val id = NativeBridge.getCommentId(ref)
        val content = NativeBridge.getCommentContent(ref)
        val authorId = NativeBridge.getCommentAuthorId(ref)
        return Comment(id, content, authorId)
    }
fun fromJniProposal(ptr: Long): Proposal {
        val id = NativeBridge.getProposalId(ptr)
        val title = NativeBridge.getProposalTitle(ptr)
        val description = NativeBridge.getProposalDescription(ptr)
        val authorId = NativeBridge.getProposalAuthorId(ptr)
        return Proposal(id, title, description, authorId)
    }

    fun fromJniFeedItem(ptr: Long): FeedItem {
        val id = NativeBridge.getFeedItemId(ptr)
        val type = NativeBridge.getFeedItemType(ptr)
        val content = NativeBridge.getFeedItemContent(ptr)
        return FeedItem(id, type, content)
    }

    fun fromJniProduct(ptr: Long): Product {
        val id = NativeBridge.getProductId(ptr)
        val name = NativeBridge.getProductName(ptr)
        val brand = NativeBridge.getProductBrand(ptr).takeIf { it.isNotEmpty() }
        val description = NativeBridge.getProductDescription(ptr)
        val barcode = NativeBridge.getProductBarcode(ptr).takeIf { it.isNotEmpty() }
        val carbonFootprint = NativeBridge.getProductCarbonFootprint(ptr)
        val packagingType = NativeBridge.getProductPackagingType(ptr).takeIf { it.isNotEmpty() }
        val nutritionalInfo = NativeBridge.getProductNutritionalInfo(ptr).takeIf { it.isNotEmpty() }
        val manufacturer = NativeBridge.getProductManufacturer(ptr).takeIf { it.isNotEmpty() }
        val materialCost = NativeBridge.getProductMaterialCost(ptr)
        val laborCost = NativeBridge.getProductLaborCost(ptr)
        val supplier = NativeBridge.getProductSupplier(ptr).takeIf { it.isNotEmpty() }
        val currentStock = NativeBridge.getProductCurrentStock(ptr)
        val reorderLevel = NativeBridge.getProductReorderLevel(ptr)
        val supplyChainPtr = NativeBridge.getProductSupplyChain(ptr)
        val supplyChain = if (supplyChainPtr != 0L) fromJniSupplyChain(supplyChainPtr) else null
        val costAmount = NativeBridge.getProductCostAmount(ptr)
        val costCurrency = NativeBridge.getProductCostCurrency(ptr).takeIf { it.isNotEmpty() }
        val locationId = NativeBridge.getProductLocationId(ptr).takeIf { it.isNotEmpty() }
        val locationName = NativeBridge.getProductLocationName(ptr).takeIf { it.isNotEmpty() }

        // Create Money object if we have cost information
        val cost = if (costAmount != 0.0 || costCurrency != null) {
            Money(costAmount, costCurrency ?: "USD")
        } else {
            null
        }

        // Create Location object if we have location information
        val location = if (locationId != null || locationName != null) {
            WarehouseLocation(locationId ?: "", locationName ?: "")
        } else {
            null
        }

        return Product(
            id,
            name,
            brand,
            description,
            barcode,
            carbonFootprint,
            packagingType,
            nutritionalInfo,
            manufacturer,
            materialCost,
            laborCost,
            supplier,
            currentStock,
            reorderLevel,
            supplyChain,
            cost,      // New cost field
            location   // New location field
        )
    }
}