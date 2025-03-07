;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core fasl module
;;;

;;;
;;; fasl objects
;;;
(mu:intern core "%fasl-direct-object"
   (:lambda (id bytes)
     (mu:unrepr (core:%fasl-slice-to-byte-vector bytes))))

(mu:intern core "%fasl-symbol-object"
   (:lambda (id bytes)
     ()))

(mu:intern core "%fasl-vector-object"
   (:lambda (id bytes)
     ()))

(mu:intern core "%fasl-type-object-map"
   `(,(mu:cons :vector core:%fasl-vector-object)
      ,(mu:cons :symbol core:%fasl-symbol-object)
      ,(mu:cons :direct core:%fasl-direct-object)))

;;;
;;; utilities
;;;
(mu:intern core "%fasl-slice-to-byte-vector"
   (:lambda (slice)
     (mu:make-vector
      :byte
      (mu:cdr
       (mu:fix
        (:lambda (loop)
          ((:lambda (index list)
             (:if (mu:less-than index (core:vector-length slice))
                  (mu:cons
                   (mu:add 1 index)
                   (mu:append (core:%list2 list (core:%list (core:vector-ref slice index)))))
                  loop))
           (mu:car loop)
           (mu:cdr loop)))
        '(0 . ()))))))
        
;;;
;;; loader
;;;
(mu:intern core "%read-fasl-header"
   (:lambda (stream)
     (core:read stream () core:%eof%)))

(mu:intern core "%read-fasl-object"
   (:lambda (id length bytes)
      (core:apply
       (mu:cdr (core:%assq (core:%fasl-id-type id) core:%fasl-type-object-map))
       `(,id ,bytes))))

(mu:intern core "fasl-load"
   (:lambda (path)
     (:if (core:stringp path)
          ((:lambda (stream)
             ((:lambda (header)
                (:if (mu:eq core:%eof% header)
                     (core:raise path 'core:fasl-load "can't read fasl header")
                     ((:lambda (version bytes)
                        (mu:fix
                         (:lambda (index)
                           (:if (mu:less-than index (mu:vector-length bytes))
                                ((:lambda (id length)
                                   (core:%read-fasl-object
                                    id
                                    length
                                    (core:vector-slice
                                     bytes
                                     (mu:add 2 index)
                                     length))
                                   (mu:add length (mu:add 2 index)))
                                 (mu:svref bytes index)
                                 (mu:svref bytes (mu:add 1 index)))
                                index))
                         0))
                      (mu:cdr (core:%assq :version header))
                      (mu:cdr (core:%assq :bytes header)))))
              (core:%read-fasl-header stream)))
             (mu:open :file :input path :t))
          (core:raise path 'core:fasl-load "not a file path"))
     ()))
