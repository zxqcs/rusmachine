begin-garbage-collection
(assign free (const 0))
(assign scan (const 0))
(assign old (reg root))
(assign relocate-continue (label reassign-root)) (goto (label relocate-old-result-in-new))
reassign-root
(assign root (reg new)) (goto (label gc-loop))

gc-loop
(test (op =) (reg scan) (reg free))
(branch (label gc-flip))
;; note that the old here has the content of the new_cars at index that scan points at
(assign old (op vector-ref) (reg new-cars) (reg scan)) 
(assign relocate-continue (label update-car))
(goto (label relocate-old-result-in-new))

update-car
;;after the pair was relocated into new_cars/new_cdrs(copied),such as Pi,Pj
;; these index i and j is no longer legal and has to be updated
(perform (op vector-set!)
;; index i is updated to new index stored in reg new
   (reg new-cars) (reg scan) (reg new))
;; old has the content of Pj which may not been relocated yet
(assign old (op vector-ref) (reg new-cdrs) (reg scan)) 
(assign relocate-continue (label update-cdr))
(goto (label relocate-old-result-in-new))

update-cdr
;; update the new_cdrs to points at a new relocated objects or storing a non pair object itself
(perform (op vector-set!)
   (reg new-cdrs) (reg scan) (reg new))
;; increment scan to enter a next iteration of gc-loop
(assign scan (op +) (reg scan) (const 1)) 
(goto (label gc-loop))

relocate-old-result-in-new
(test (op pointer-to-pair?) (reg old))
 (branch (label pair))
;; since the content reg old points at is not a pair such as 3 in new_cars/cdrs
;; there is no need to relocate from the_cars/cdrs to new_cars/cdrs
;; just let reg new copy the content of reg old such as 3
(assign new (reg old))
(goto (reg relocate-continue))
pair
;; note that here reg old has the contet such as Pi which points at the ith index in the_cars
;; such that this assign operation use this i index to fetch contents
;; in ith positon of the_cars and put it in oldcr
(assign oldcr (op vector-ref) (reg the-cars) (reg old))
(test (op broken-heart?) (reg oldcr)) 
(branch (label already-moved))
(assign new (reg free)) ; new location for pair 
;; Update free pointer.
(assign free (op +) (reg free) (const 1)) 
;; Copy the car and cdr to new memory.
(perform (op vector-set!)
    (reg new-cars) (reg new) (reg oldcr)
(assign oldcr (op vector-ref) (reg the-cdrs) (reg old)) 
(perform (op vector-set!)
    (reg new-cdrs) (reg new) (reg oldcr)) 
;; Construct the broken heart.
(perform (op vector-set!)
(reg the-cars) (reg old) (const broken-heart))
(perform
    (op vector-set!) (reg the-cdrs) (reg old) (reg new))
    (goto (reg relocate-continue)) 

already-moved
;; note that here reg old has the content such as Pi which points at the ith index in 
;; the_cars, since a broken heart is found at that position, it contunues to fetch the 
;; new position of that item in new_cars/new_cdrs which is stored in the ith index of 
;; the_cdrs
(assign new (op vector-ref) (reg the-cdrs) (reg old)) 
(goto (reg relocate-continue))


gc-flip
(assign temp (reg the-cdrs))
(assign the-cdrs (reg new-cdrs)) 
(assign new-cdrs (reg temp)) 
(assign temp (reg the-cars))
(assign the-cars (reg new-cars)) 
(assign new-cars (reg temp))
