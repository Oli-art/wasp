// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;
use crate::types::*;

// Defining default configuration parameters

// The default amount of players to tag an image for it to be considered processed
const DEFAULT_TAGS_REQUIRED_PER_IMAGE: i32 = 10;
// The distance required for two clusters of tags to remain separate and not metge into one
const MIN_INTER_CLUSTER_DISTANCE: f64 = 100.0;
// The percentage of players on an image that have to agree on a tag for it to be valid
const CONFIRMATION_PERCENTAGE: f32 = 0.6;

// 'createRound' is used for a researcher to start a round for players to tag images, wich can then be used
// for training an Artificial Intelligence (Deep Learning). It requires the researcher to place a reward for
// the players efforts. If the players tag correctly an image, based on other plays, it gets rewarded.
// The 'createRound' function takes 2 mandatory parameters:
// - 'numberOfImages', which must be an Int32 number, and
// - 'description', which must be a String
// It can also take a non-mandatory parameter:
// - 'tagsRequiredPerImage', which must be an Int16
pub fn func_create_round(_ctx: &ScFuncContext, _f: &CreateRoundContext) {

    // No round can be in progress in order to be able to crate a new one
    _ctx.require(_f.state.reward().value() == 0_i64, "Error: Round already in progress");

    // Create ScBalances proxy to the incoming balances for this request.
    let incoming: ScBalances = _ctx.incoming();

    // Set the state variables of the round.
    _f.state.processed_images().clear();
    let reward = incoming.balance(&ScColor::IOTA);
    let number_of_images = _f.params.number_of_images().value();
    let description = &_f.params.description().value();
    let mut tags_required_per_image = _f.params.tags_required_per_image().value();

    if tags_required_per_image < 1 {
        tags_required_per_image = DEFAULT_TAGS_REQUIRED_PER_IMAGE;
    }
    _f.state.reward().set_value(reward);
    _f.state.number_of_images().set_value(number_of_images);
    _f.state.description().set_value(description);
    _f.state.tags_required_per_image().set_value(tags_required_per_image);
    _f.state.creator().set_value(&_ctx.caller());
    
    // Reward must be at least one iota per tag
    _ctx.require(reward >= tags_required_per_image as i64 * number_of_images as i64, "Error: Reward too low!");

    _ctx.event(&format!(
        "round.started {0} {1} {2} {3}",
        number_of_images,
        tags_required_per_image,
        reward.to_string(),
        description
    ));
}

// This function can only be executed by the creator of the round (alias: the investigator).
// Ideally, it should be executed when all images have been tagged the required amount of times,
// although it can be executed before that. What this function does, is to arrange the tags of 
// the individual images into clusters and finding outliers this way. Outliers are tags that are 
// not inside of a 'big' cluster (defined by the CONFIRMATION_PERCENTAGE constant). They represent 
// a bad tag and are not considered valid. For every valid tag, it's corresponding player recieves 
// a reward. This reward is the monney put into the contract by the investigator, and it gets 
// split equally between valid tags. We calculate the center of the clusters and find the top best 
// tags by finding the shortest distances from this tags to the center of their cluster. The 
// betters that placed this tags, win the betting monney, apart from the default reward for making 
// valid tags. 
// The 'endRound' function takes no parameters.
pub fn func_end_round(_ctx: &ScFuncContext, _f: &EndRoundContext) {

    // The context caller has to be the round crator.
    _ctx.require(_f.state.creator().value() == _ctx.caller(), 
    "Error: Only the round creator can end the round");

    // Also, a round has to be in progress.
    _ctx.require(_f.state.reward().value() != 0_i64,
    "Error: No round in progress");

    // We will need this function later to calculate distances between points in four dimentions (x, y, h, w)
    fn euclidean_distance(a: Vec<i64>, b: Vec<i64>) -> f64 {
        (((a[0]-b[0])*(a[0]-b[0]) + (a[1]-b[1])*(a[1]-b[1]) + (a[2]-b[2])*(a[2]-b[2]) + (a[3]-b[3])*(a[3]-b[3])) as f64).sqrt()
    }

    // Iterate across all the images, and save: 
    // - the centers of the clusters for every image
    // - the players realted to the valid tags

    struct ValidTag {
        player: ScAgentID,
        tagged_image_id: i32
    }

    let mut valid_tags: Vec<ValidTag> = Vec::new();      // stores the player and imageId of all valid tags
    let mut centers: Vec<Vec<TaggedImage>> = Vec::new(); // stores the center of the clusters for all images

    // retrieve some state variables
    let number_of_images = _f.state.number_of_images().value();
    let tags_req_per_image = _f.state.tags_required_per_image().value() as i32;

    // For every image, we apply Aglomerative Hierarchical Clustering
    for image in 0..number_of_images {
        let mut clusters:Vec<Vec<i64>> = Vec::new(); // stores clusters with their centers and all the id's of the point's that conform it
        let mut plays_for_this_image = 0; // counts the real amount of players that tagged this image. This is because
                                          // the round could end before images are tagged with the required amount 
                                          // it will be used to calculate the amount of players needed to agree for a valid tag
        for i in image*tags_req_per_image..(image+1)*tags_req_per_image {  // I'm forced to do this is because there are no nested arrays in schema yet
            if !_f.state.tagged_images().get_tagged_image(i).exists() { break }
            let tagged_image = _f.state.tagged_images().get_tagged_image(i).value();
            // Every 'tagged_image' starts as one cluster. The algorithm will then merge close-by clusters
            let cluster = vec![tagged_image.x, tagged_image.y, tagged_image.h, tagged_image.w, i as i64];
            clusters.push(cluster);
            plays_for_this_image +=1;
        }

        // every tag starts as a different cluster. We merge them until they are more than 100 pixels⁴ apart.
        let mut min_distance= [0.0, 0.0, 0.0]; // stores [distance between two clusters, 1st cluster, 2nd cluster]

        // Here, we apply the Aglomerative Hierarchical Clustering: Merging all clusters that are the closest to each other
        // until the closest are more than MIN_INTER_CLUSTER_DISTANCE pixels⁴ or there is only one cluster left (in that
        // case, 9999999.0 will not be overwritten).
        while min_distance[0] <= MIN_INTER_CLUSTER_DISTANCE {
            // Evaluate the distance matrix and store the shortest euclidean distance in 'min_distance[0]'
            min_distance[0]= 9999999.0;
            for x in 0..clusters.len() {
                for y in x+1..clusters.len() { // this way we dont evaluete a pair twice, nor a cluster against itself
                    let distance = euclidean_distance(clusters[x].clone(), clusters[y].clone());
                    if distance < min_distance[0] {
                        min_distance = [distance, x as f64, y as f64];
                    }
                }
            }

            // If the four dimentional distance is greter than 100, then we dont merge the clusters.
            // Points that are this far away are considered different final clusters
            if min_distance[0] < MIN_INTER_CLUSTER_DISTANCE {

                // define the indexes of the clusters one and two to be merged
                let index_1 = min_distance[1] as usize;
                let index_2 = min_distance[2] as usize;
                // the weight is equal to the number of point's that conform the cluster
                let weight_1 = (clusters[index_1].len() - 4) as i64;
                let weight_2 = (clusters[index_2].len() - 4) as i64;
                
                // Calculating the coordiantes of the new cluster. The more weight, 
                // the more influence on the new coordinate it has. This way, the 
                // coordinate represents the average of all points in the cluster
                // TODO: Divisions inside a loop are not cool. Maybe we can improve this somehow?
                let mut new_cluster = vec![
                    (clusters[index_1][0] * weight_1 + clusters[index_2][0] * weight_2)/(weight_1 + weight_2),
                    (clusters[index_1][1] * weight_1 + clusters[index_2][1] * weight_2)/(weight_1 + weight_2),
                    (clusters[index_1][2] * weight_1 + clusters[index_2][2] * weight_2)/(weight_1 + weight_2),
                    (clusters[index_1][3] * weight_1 + clusters[index_2][3] * weight_2)/(weight_1 + weight_2)
                ];
                // Push the point indexes inside both clusters to the new cluster
                for i in 0..weight_1 {
                    new_cluster.push(clusters[index_1][i as usize + 4]);
                }
                for i in 0..weight_2 {
                    new_cluster.push(clusters[index_2][i as usize + 4]);
                }

                // Remove the old clusters and replace with the new one. Note that inxex_2 > index_1.
                // When removing index_2 first, we don't alter index_1.
                clusters.remove(index_2);
                clusters.remove(index_1);
                clusters.push(new_cluster);
            }
        } // finish Aglomerative Hierarchical Clustering Algorithm

        // We should be left with only one cluster (until multi-tagging is implemented). 
        // The ones that have fewer points get discarted.
        // Here we also store all the players that made correct taggs. They can be stored multiple times.
        let length = clusters.len();
        let mut check_min_one_tag = false; // this is expĺained below the for loop
        for i in 0..length {
            let id = length-i-1; // this way it's a backwards iterator and we dont change the id's as we remove them.
            if clusters[id].len() -4 <= (plays_for_this_image as f32 * CONFIRMATION_PERCENTAGE) as usize {
                clusters.remove(i);
            } else { // here we push the players that tagged correctly to the reward-list
                for j in 4..clusters[id].len() {
                    let vaid_tag = ValidTag{
                        player: _f.state.tagged_images().get_tagged_image(clusters[id][j] as i32).value().player,
                        tagged_image_id: clusters[id][j] as i32
                    };
                    valid_tags.push(vaid_tag);
                }
                check_min_one_tag = true;
            }
        }
        // We want to have one cluster per image, even if it is an empty cluster. This way,
        // it's easier to find processed images based on their id. TODO: With nested arrays and nested
        // constructors, this abomination would not be necessary. Also, this is a hackaton, so no time...
        if !check_min_one_tag {
            let cluster = vec![-1, -1, -1, -1];
            clusters.push(cluster);
        }

        // We append the clusters coordinate to the centers vector (a vector of centers for every image)
        let mut image_centers: Vec<TaggedImage> = Vec::new(); 
        // TODO: We only have one cluster left, so a for loop is not really necessary until we have multi-tagging
        for i in clusters {
            let center = TaggedImage {
                player: _f.state.creator().value(), // the constructor requires a creator. This time it's not used tho.
                image_id: image,
                x: i[0],
                y: i[1],
                h: i[2],
                w: i[3]
            };
            image_centers.push(center);
        }
        centers.push(image_centers);
    } // finish the images iterator

    // The following line, sorts the centers vector by 'image_id'
    centers.sort_by(|a, b| b[0].image_id.cmp(&a[0].image_id));
    // update the 'processed_images' state variable with the final tagging data
    for i in centers{
        for j in i{
            _f.state.processed_images().get_tagged_image(j.image_id).set_value(&j)
        }
    }

    // Now, we pay all the players for their respective valid tags.
    // This is done by splitting the reward money placed by the creator.
    let n_rewards = valid_tags.len() as i64;
    let transfers: ScTransfers = ScTransfers::iotas(_f.state.reward().value()/n_rewards);
    for valid_tag in &valid_tags {
        // Transfer the reward to players who tagged correctly
        _ctx.transfer_to_address(&valid_tag.player.address(), transfers);
    }

    // Now, we set the top players and the rewards for the correct tags
    // The betters_top vector is an ordered list of the winners, from better to worse tagger.
    struct Better {
        accuracy: f64,
        player: ScAgentID,
        amount: i64
    }
    impl Better {
        pub fn new(accuracy: f64, player: ScAgentID, amount:i64) -> Self {
            Better {
                accuracy,
                player,
                amount
            }
        }
    }

    // 'valid_bets' stores all the bets placed, including zero value ones (with the player, 
    // the accuracy of the tag and, for the moment, a total bet equal to zero)
    let mut valid_bets: Vec<Better> = Vec::new();
    // fill the 'valid_bets' with the bets. The bet amount will be filled later 
    for valid_tag in valid_tags {
        let tagged_image = _f.state.tagged_images().get_tagged_image(valid_tag.tagged_image_id).value();
        let tagged_image_point = vec![tagged_image.x, tagged_image.y, tagged_image.h, tagged_image.w];
        let cluster_center = _f.state.processed_images().get_tagged_image(tagged_image.image_id).value();
        let cluster_center_point = vec![cluster_center.x, cluster_center.y, cluster_center.h, cluster_center.w];
        let distance_to_cluster_center = euclidean_distance(tagged_image_point, cluster_center_point);
        valid_bets.push(Better::new(distance_to_cluster_center, tagged_image.player, 0));
    }

    // We now have a list with all the betters that made a valid tag, but they are repeated.
    // Next, we make a new list, with no repeated players, and with their best accuracy!
    let mut betters_top: Vec<Better> = Vec::new();
    'all: for valid_bet in valid_bets {
        for better in 0..betters_top.len() {
            if valid_bet.player == betters_top[better].player {
                // replace the accuracy for the player's best one
                if valid_bet.accuracy > betters_top[better].accuracy{
                    betters_top[better].accuracy = valid_bet.accuracy;
                }
                // ship to next iteration of the outer loop to avoid adding the player to the 'betters_top' again
                continue 'all;
            }
        }
        betters_top.push(valid_bet);
    }

    // Next, we calculate the total amount of iotas betted by the players in the 'betters_top' list
    'bet: for i in 0.._f.state.bets().length() {
        let bet = _f.state.bets().get_bet(i).value();
        for better in 0..betters_top.len() {
            if bet.player == betters_top[better].player {
                betters_top[better].amount += bet.amount;
                continue 'bet;
            }
        }
    }

    // sort the 'top_betters' by the accuracy
    betters_top.sort_by(|a, b| b.accuracy.partial_cmp(&a.accuracy).unwrap());

    // Finding the total value placed in the round's bets
    let mut total_payout: i64 = 0_i64;
    for bet in &betters_top {
        total_payout += bet.amount;
    }
    // 'points' represents by how much the betting money has to be divided.
    // We have to fit the amount betted to the sum of all the prices 
    let mut points: i64 = 0_i64;
    for i in 0..betters_top.len() {
        // The prices take an exponential form, where the 'i' represents the position of the player given it's acuracy.
        points += ((betters_top.len()-i)*(betters_top.len()-i)) as i64;
    }
    let multiplier: i64 = total_payout/points; 

    // here we calculate how much to betting monney to transfer to every player, and we tranfer it
    // TODO: rounding errors could happen
    for i in 0..betters_top.len()-1 {
        // Again, the prices take an exponential form, where the 'i' 
        // represents the position of the player given it's acuracy.
        let payout = multiplier*((betters_top.len()-i)*(betters_top.len()-i)) as i64;
        let transfers: ScTransfers = ScTransfers::iotas(payout);
        _ctx.transfer_to_address(&betters_top[i].player.address(), transfers);
    }

    // We clear all the state variables, so a new round can begin
    _f.state.bets().clear();
	_f.state.plays_per_image().clear();
	_f.state.tagged_images().clear();
    _f.state.reward().set_value(0_i64);
    _f.state.pending_plays().clear();
}

// This function is used by players to be assigned an image and for them to place a bet on their tags.
// If an image has all the tags it requires by the 'tags_required_per_image' variable, this image can no
// longer be assigned to a player. Also, if all images have their required tags, no image can be assigned at all.
// The 'requestPlay' function takes no parameters.
pub fn func_request_play(_ctx: &ScFuncContext, _f: &RequestPlayContext) {
    
    // Defining relevant variables for the request 
    let tags_required_per_image = _f.state.tags_required_per_image().value();
    let number_of_images = _f.state.number_of_images().value();
    let player = _ctx.caller();
    let plays_per_image = _f.state.plays_per_image();
    let pending_plays = _f.state.pending_plays();

    // Check if the player has an open request. If it does, panic. 
    for i in 0..pending_plays.length() {
        if pending_plays.get_bet(i).value().player == _ctx.caller() {
            panic("Error: Player already has an open request");
        }
    }

    // Check if any images are available for the player to tag. If all are tagged the required amount of times
    // or if the ones available have been already tagged by the player, the counter will be equal to the number of images.
    let mut counter: i32 = 0;
    'image: for i in 0..plays_per_image.length() {
        if plays_per_image.get_int32(i).value() >= tags_required_per_image {
            counter += 1;
            continue;
        }
        for j in i..i+tags_required_per_image as i32 {
            if _f.state.tagged_images().get_tagged_image(j).value().player.address() == player.address() {
                counter +=1;
                continue 'image;
            }
        }
    }

    // If no more images are available to tag, we dont accept the request and panic.
    _ctx.require(counter != number_of_images, "Error: Sorry, no more images are available to tag");

    // We choose an image randomly to assign to the player for tagging.
    // This loop checks if the image has been tagged the required amount of times, 
    // or if it has already been tagged by the player. If so, we choose another one.
    // Note that the loop is not infinite, as we have checked that there is at least an image available to tag.
    let mut image_id: i32;
    loop {
        image_id = _ctx.utility().random((number_of_images-1) as i64) as i32;
        if plays_per_image.get_int32(image_id).value() == tags_required_per_image { continue }
        for i in image_id..image_id+tags_required_per_image as i32 {
            if  _f.state.tagged_images().get_tagged_image(i).exists() { 
                if _f.state.tagged_images().get_tagged_image(i).value().player.address() == player.address() {
                    continue
                }
            }
        }
        break
    }


    // Create ScBalances proxy to the incoming balances for this request.
    let incoming: ScBalances = _ctx.incoming();
    let bet = Bet {
        amount: incoming.balance(&ScColor::IOTA),
        player: _ctx.caller(),
        image_id: image_id,
    };

    // Append the bet data to the bets array and to the pending plays array. 
    // They will automatically take care of serializing the bet struct into a bytes representation.
    let bets: ArrayOfMutableBet = _f.state.bets();
    let bets_nr: i32 = bets.length();
    bets.get_bet(bets_nr).set_value(&bet);
    let pending_plays_nr: i32 = pending_plays.length();
    pending_plays.get_bet(pending_plays_nr).set_value(&bet);

    _ctx.event(&format!(
        "play.requested {0} {1} {2} {3}",
        &bet.player.address().to_string(),
        bet.amount,
        bet.image_id,
        _f.state.pending_plays().length().to_string()
    ));

    _f.results.image_id().set_value(image_id);
}

// This function is used for a player to tag an image that has been assigned to it. 
// It basically deletes the request from the request list 'pending_plays', adds the
// information of the tag to the tagged images list and adds one to the number of times
// the image has been tagged, using the 'plays_per_round' list.
// The 'sendTags' function takes 4 mandatory parameters, corresponding to the coordinates and dimentions of the tag:
// - 'x', which must be an Int64 number,
// - 'y', which must be an Int64 number, 
// - 'h', which must be an Int64 number and
// - 'w', which must be an Int64 number
pub fn func_send_tags(_ctx: &ScFuncContext, _f: &SendTagsContext) {

    let pending_plays: ArrayOfMutableBet = _f.state.pending_plays();
    let mut bet: Option<MutableBet> = None;
    let mut pending_play_id: i32 = 0;
    
    // Searching for the player's open request. If it doesn't exist, panic.
    // If it does, it will get stored as an option. We will have to use unwrap() to access it
    for i in 0..pending_plays.length() {
        if pending_plays.get_bet(i).value().player == _ctx.caller() {
            bet = Some(pending_plays.get_bet(i));
            pending_play_id = i;
        }
    }
    if bet.is_none() {
        _ctx.panic("Error: No plays requested for this address");
    }

    // We delete the bet from the pending plays by clearing the array and copying again, minus the bet of the player
    _f.state.pending_plays().clear();
    for i in 0..pending_play_id {
        _f.state.pending_plays().get_bet(i).set_value(&pending_plays.get_bet(i).value());
    }
    for i in pending_play_id+1..pending_plays.length() {
        _f.state.pending_plays().get_bet(i-1).set_value(&pending_plays.get_bet(i).value());
    }

    // Get the image_id and the number of times a play has been made for this image.
    let image_id = bet.unwrap().value().image_id;
    let plays_per_image: i32 = _f.state.plays_per_image().get_int32(image_id).value();

    // If the image has all it's required plays, we panic. 
    // Note that the request has been removed from the pendingPlays list
    if plays_per_image >= _f.state.tags_required_per_image().value() {
        _ctx.panic("Error: All plays have been made for this image. Please request another one.");
    }

    // We gather all the information into this struct
    let tagged_image = TaggedImage {
        image_id: image_id,
        player: _ctx.caller(),
        x: _f.params.x().value(),
        y: _f.params.y().value(),
        h: _f.params.h().value(),
        w: _f.params.w().value()
    };

    // Append the bet data to the bets array. The bet array will automatically take care
    // of serializing the bet struct into a bytes representation.
    _f.state.tagged_images().get_tagged_image(image_id + plays_per_image).set_value(&tagged_image);

    // Add one to the number of times this image has been tagged
    let plays_for_this_image: i32 = _f.state.plays_per_image().get_int32(tagged_image.image_id).value();
    _f.state.plays_per_image().get_int32(tagged_image.image_id).set_value(plays_for_this_image + 1);

    _ctx.event(&format!(
        "dtag.image.tagged {0} {1} {2}",
        &tagged_image.player.address().to_string(),
        _f.state.plays_per_image().get_int32(tagged_image.image_id).value().to_string(),
        _f.state.pending_plays().length().to_string()
    ));
}


pub fn view_get_plays_per_image(_ctx: &ScViewContext, _f: &GetPlaysPerImageContext) {

    let image_id = _f.params.image_id().value();
    let plays = _f.state.plays_per_image().get_int32(image_id).value();

    _f.results.plays_per_image().set_value(plays);
}

pub fn view_get_results(_ctx: &ScViewContext, _f: &GetResultsContext) {

    let image_id = _f.params.image_id().value();
    let tagged_image = _f.state.processed_images().get_tagged_image(image_id).value();

    _f.results.x().set_value(tagged_image.x);
    _f.results.y().set_value(tagged_image.y);
    _f.results.h().set_value(tagged_image.h);
    _f.results.w().set_value(tagged_image.w);
}
