
�

auth.protoauth"
_LoginRequest"�
_LoginResponseK
direct_browser (2".auth._LoginResponse.DirectBrowserH RdirectBrowser<
	logged_in (2.auth._LoginResponse.LoggedInH RloggedIn8
message (2.auth._LoginResponse.MessageH Rmessage2
error (2.auth._LoginResponse.ErrorH Rerror[
LoggedIn#
session_token (	RsessionToken*
valid_for_seconds (RvalidForSeconds)
Error 
description (	Rdescription!
DirectBrowser
url (	Rurl
Message
text (	RtextB
state2>
Auth6
Login.auth._LoginRequest.auth._LoginResponse" 0B
momento.authPJ�

  2

  

 "
	

 "

 %
	
 %

 


  	


 

  >

  

  

  %+

  ,:


  


 


 2




 

 

 %

 

  

 #$



















	





-
 " Terminal state, login success


 


  

  


  

  
�
 !!� How many seconds the token was valid for when issued.
 Will vary slightly from reality upon receipt, as
 time has passed since the token was minted.
 You might expect to see this true to within 10
 seconds of client-side timekeeping but as is
 ever the case, there are no guarantees with
 public network timing.


 !


 !

 ! 
+
%' Terminal state, login error


%


 &

 &


 &

 &
>
*,0 Open a browser to a url, for interactive login


*


 +

 +


 +

 +
4
/1& Logging info about the login process


/


 0

 0


 0

 0bproto3
��
cacheclient.protocache_client"*
_GetRequest
	cache_key (RcacheKey"{
_GetResponse2
result (2.cache_client.ECacheResultRresult

cache_body (R	cacheBody
message (	Rmessage"-
_DeleteRequest
	cache_key (RcacheKey"
_DeleteResponse"t
_SetRequest
	cache_key (RcacheKey

cache_body (R	cacheBody)
ttl_milliseconds (RttlMilliseconds"\
_SetResponse2
result (2.cache_client.ECacheResultRresult
message (	Rmessage"
_SetIfNotExistsRequest
	cache_key (RcacheKey

cache_body (R	cacheBody)
ttl_milliseconds (RttlMilliseconds"�
_SetIfNotExistsResponseG
stored (2-.cache_client._SetIfNotExistsResponse._StoredH RstoredQ

not_stored (20.cache_client._SetIfNotExistsResponse._NotStoredH R	notStored	
_Stored

_NotStoredB
result"s
_IncrementRequest
	cache_key (RcacheKey
amount (Ramount)
ttl_milliseconds (RttlMilliseconds"*
_IncrementResponse
value (Rvalue"X
_DictionaryGetRequest'
dictionary_name (RdictionaryName
fields (Rfields"�
_DictionaryGetResponseC
found (2+.cache_client._DictionaryGetResponse._FoundH RfoundI
missing (2-.cache_client._DictionaryGetResponse._MissingH Rmissingo
_DictionaryGetResponsePart2
result (2.cache_client.ECacheResultRresult

cache_body (R	cacheBody_
_FoundU
items (2?.cache_client._DictionaryGetResponse._DictionaryGetResponsePartRitems

_MissingB

dictionary"B
_DictionaryFetchRequest'
dictionary_name (RdictionaryName"G
_DictionaryFieldValuePair
field (Rfield
value (Rvalue"�
_DictionaryFetchResponseE
found (2-.cache_client._DictionaryFetchResponse._FoundH RfoundK
missing (2/.cache_client._DictionaryFetchResponse._MissingH RmissingG
_Found=
items (2'.cache_client._DictionaryFieldValuePairRitems

_MissingB

dictionary"�
_DictionarySetRequest'
dictionary_name (RdictionaryName=
items (2'.cache_client._DictionaryFieldValuePairRitems)
ttl_milliseconds (RttlMilliseconds
refresh_ttl (R
refreshTtl"
_DictionarySetResponse"�
_DictionaryIncrementRequest'
dictionary_name (RdictionaryName
field (Rfield
amount (Ramount)
ttl_milliseconds (RttlMilliseconds
refresh_ttl (R
refreshTtl"4
_DictionaryIncrementResponse
value (Rvalue"�
_DictionaryDeleteRequest'
dictionary_name (RdictionaryNameA
some (2+.cache_client._DictionaryDeleteRequest.SomeH Rsome>
all (2*.cache_client._DictionaryDeleteRequest.AllH Rall
Some
fields (Rfields
AllB
delete"
_DictionaryDeleteResponse"-
_SetFetchRequest
set_name (RsetName"�
_SetFetchResponse>
found (2&.cache_client._SetFetchResponse._FoundH RfoundD
missing (2(.cache_client._SetFetchResponse._MissingH Rmissing$
_Found
elements (Relements

_MissingB
set"�
_SetUnionRequest
set_name (RsetName
elements (Relements)
ttl_milliseconds (RttlMilliseconds
refresh_ttl (R
refreshTtl"
_SetUnionResponse"�
_SetDifferenceRequest
set_name (RsetNameH
minuend (2,.cache_client._SetDifferenceRequest._MinuendH RminuendQ

subtrahend (2/.cache_client._SetDifferenceRequest._SubtrahendH R
subtrahend&
_Minuend
elements (Relements�
_SubtrahendH
set (24.cache_client._SetDifferenceRequest._Subtrahend._SetH RsetW
identity (29.cache_client._SetDifferenceRequest._Subtrahend._IdentityH Ridentity"
_Set
elements (Relements
	_IdentityB
subtrahend_setB

difference"�
_SetDifferenceResponseC
found (2+.cache_client._SetDifferenceResponse._FoundH RfoundI
missing (2-.cache_client._SetDifferenceResponse._MissingH Rmissing
_Found

_MissingB
set"�
_ListConcatenateFrontRequest
	list_name (RlistName
values (Rvalues)
ttl_milliseconds (RttlMilliseconds
refresh_ttl (R
refreshTtl1
truncate_back_to_size (RtruncateBackToSize"@
_ListConcatenateFrontResponse
list_length (R
listLength"�
_ListConcatenateBackRequest
	list_name (RlistName
values (Rvalues)
ttl_milliseconds (RttlMilliseconds
refresh_ttl (R
refreshTtl3
truncate_front_to_size (RtruncateFrontToSize"?
_ListConcatenateBackResponse
list_length (R
listLength"�
_ListPushFrontRequest
	list_name (RlistName
value (Rvalue)
ttl_milliseconds (RttlMilliseconds
refresh_ttl (R
refreshTtl1
truncate_back_to_size (RtruncateBackToSize"9
_ListPushFrontResponse
list_length (R
listLength"�
_ListPushBackRequest
	list_name (RlistName
value (Rvalue)
ttl_milliseconds (RttlMilliseconds
refresh_ttl (R
refreshTtl3
truncate_front_to_size (RtruncateFrontToSize"8
_ListPushBackResponse
list_length (R
listLength"3
_ListPopFrontRequest
	list_name (RlistName"�
_ListPopFrontResponseB
found (2*.cache_client._ListPopFrontResponse._FoundH RfoundH
missing (2,.cache_client._ListPopFrontResponse._MissingH Rmissing
_Found
front (Rfront

_MissingB
list"2
_ListPopBackRequest
	list_name (RlistName"�
_ListPopBackResponseA
found (2).cache_client._ListPopBackResponse._FoundH RfoundG
missing (2+.cache_client._ListPopBackResponse._MissingH Rmissing
_Found
back (Rback

_MissingB
list"C

_ListRange
begin_index (R
beginIndex
count (Rcount"�
_ListEraseRequest
	list_name (RlistNameA
some (2+.cache_client._ListEraseRequest._ListRangesH Rsome8
all (2$.cache_client._ListEraseRequest._AllH Rall
_All?
_ListRanges0
ranges (2.cache_client._ListRangeRrangesB
erase"
_ListEraseResponse"t
_ListRemoveRequest
	list_name (RlistName7
all_elements_with_value (H RallElementsWithValueB
remove"
_ListRemoveResponse"g
_ListFetchRequest
	list_name (RlistName
start_index (R
startIndex
count (Rcount"�
_ListFetchResponse?
found (2'.cache_client._ListFetchResponse._FoundH RfoundE
missing (2).cache_client._ListFetchResponse._MissingH Rmissing 
_Found
values (Rvalues

_MissingB
list"1
_ListLengthRequest
	list_name (RlistName"�
_ListLengthResponse@
found (2(.cache_client._ListLengthResponse._FoundH RfoundF
missing (2*.cache_client._ListLengthResponse._MissingH Rmissing 
_Found
length (Rlength

_MissingB
list"�
_ListRetainRequest
	list_name (RlistName
count (Rcount
start_index (R
startIndex)
ttl_milliseconds (RttlMilliseconds
refresh_ttl (R
refreshTtl"
_ListRetainResponse"=
_SortedSetElement
name (Rname
score (Rscore"�
_SortedSetPutRequest
set_name (RsetName;
elements (2.cache_client._SortedSetElementRelements)
ttl_milliseconds (RttlMilliseconds
refresh_ttl (R
refreshTtl"
_SortedSetPutResponse"�
_SortedSetFetchRequest
set_name (RsetName@
order (2*.cache_client._SortedSetFetchRequest.OrderRorder=
all (2).cache_client._SortedSetFetchRequest._AllH RallC
limit (2+.cache_client._SortedSetFetchRequest._LimitH RlimitJ
by_index (2-.cache_client._SortedSetFetchRequest._ByIndexHRbyIndexJ
by_score (2-.cache_client._SortedSetFetchRequest._ByScoreHRbyScore
_All
_Limit
limit (Rlimit

_ByIndex�
_ByScoreM
start (27.cache_client._SortedSetFetchRequest._ByScore._EndpointRstartK
stop (27.cache_client._SortedSetFetchRequest._ByScore._EndpointRstop�
	_EndpointV
score (2>.cache_client._SortedSetFetchRequest._ByScore._Endpoint._ScoreH RscoreP
inf (2<.cache_client._SortedSetFetchRequest._ByScore._Endpoint._InfH Rinf2
_Score
score (Rscore
open (Ropen
_InfB
kind"&
Order
	ASCENDING 

DESCENDINGB
num_resultsB
range"�
_SortedSetFetchResponseD
found (2,.cache_client._SortedSetFetchResponse._FoundH RfoundJ
missing (2..cache_client._SortedSetFetchResponse._MissingH RmissingE
_Found;
elements (2.cache_client._SortedSetElementRelements

_MissingB

sorted_set"Y
_SortedSetGetScoreRequest
set_name (RsetName!
element_name (RelementName"�
_SortedSetGetScoreResponseP
found (28.cache_client._SortedSetGetScoreResponse._SortedSetFoundH RfoundV
missing (2:.cache_client._SortedSetGetScoreResponse._SortedSetMissingH Rmissingj
_SortedSetGetScoreResponsePart2
result (2.cache_client.ECacheResultRresult
score (Rscorev
_SortedSetFoundc
elements (2G.cache_client._SortedSetGetScoreResponse._SortedSetGetScoreResponsePartRelements
_SortedSetMissingB

sorted_set"�
_SortedSetRemoveRequest
set_name (RsetName>
all (2*.cache_client._SortedSetRemoveRequest._AllH RallA
some (2+.cache_client._SortedSetRemoveRequest._SomeH Rsome
_All*
_Some!
element_name (RelementNameB
remove_elements"
_SortedSetRemoveResponse"�
_SortedSetIncrementRequest
set_name (RsetName!
element_name (RelementName
amount (Ramount)
ttl_milliseconds (RttlMilliseconds
refresh_ttl (R
refreshTtl"3
_SortedSetIncrementResponse
value (Rvalue"X
_SortedSetGetRankRequest
set_name (RsetName!
element_name (RelementName"�
_SortedSetGetRankResponse^
element_rank (29.cache_client._SortedSetGetRankResponse._RankResponsePartH RelementRankU
missing (29.cache_client._SortedSetGetRankResponse._SortedSetMissingH Rmissing[
_RankResponsePart2
result (2.cache_client.ECacheResultRresult
rank (Rrank
_SortedSetMissingB
rank*<
ECacheResult
Invalid 
Ok
Hit
Miss"2�
Scs>
Get.cache_client._GetRequest.cache_client._GetResponse" >
Set.cache_client._SetRequest.cache_client._SetResponse" _
SetIfNotExists$.cache_client._SetIfNotExistsRequest%.cache_client._SetIfNotExistsResponse" G
Delete.cache_client._DeleteRequest.cache_client._DeleteResponse" P
	Increment.cache_client._IncrementRequest .cache_client._IncrementResponse" \
DictionaryGet#.cache_client._DictionaryGetRequest$.cache_client._DictionaryGetResponse" b
DictionaryFetch%.cache_client._DictionaryFetchRequest&.cache_client._DictionaryFetchResponse" \
DictionarySet#.cache_client._DictionarySetRequest$.cache_client._DictionarySetResponse" n
DictionaryIncrement).cache_client._DictionaryIncrementRequest*.cache_client._DictionaryIncrementResponse" e
DictionaryDelete&.cache_client._DictionaryDeleteRequest'.cache_client._DictionaryDeleteResponse" M
SetFetch.cache_client._SetFetchRequest.cache_client._SetFetchResponse" M
SetUnion.cache_client._SetUnionRequest.cache_client._SetUnionResponse" \
SetDifference#.cache_client._SetDifferenceRequest$.cache_client._SetDifferenceResponse" \
ListPushFront#.cache_client._ListPushFrontRequest$.cache_client._ListPushFrontResponse" Y
ListPushBack".cache_client._ListPushBackRequest#.cache_client._ListPushBackResponse" Y
ListPopFront".cache_client._ListPopFrontRequest#.cache_client._ListPopFrontResponse" V
ListPopBack!.cache_client._ListPopBackRequest".cache_client._ListPopBackResponse" P
	ListErase.cache_client._ListEraseRequest .cache_client._ListEraseResponse" S

ListRemove .cache_client._ListRemoveRequest!.cache_client._ListRemoveResponse" P
	ListFetch.cache_client._ListFetchRequest .cache_client._ListFetchResponse" S

ListLength .cache_client._ListLengthRequest!.cache_client._ListLengthResponse" q
ListConcatenateFront*.cache_client._ListConcatenateFrontRequest+.cache_client._ListConcatenateFrontResponse" n
ListConcatenateBack).cache_client._ListConcatenateBackRequest*.cache_client._ListConcatenateBackResponse" S

ListRetain .cache_client._ListRetainRequest!.cache_client._ListRetainResponse" Y
SortedSetPut".cache_client._SortedSetPutRequest#.cache_client._SortedSetPutResponse" _
SortedSetFetch$.cache_client._SortedSetFetchRequest%.cache_client._SortedSetFetchResponse" h
SortedSetGetScore'.cache_client._SortedSetGetScoreRequest(.cache_client._SortedSetGetScoreResponse" b
SortedSetRemove%.cache_client._SortedSetRemoveRequest&.cache_client._SortedSetRemoveResponse" k
SortedSetIncrement(.cache_client._SortedSetIncrementRequest).cache_client._SortedSetIncrementResponse" e
SortedSetGetRank&.cache_client._SortedSetGetRankRequest'.cache_client._SortedSetGetRankResponse" Bd
grpc.cache_clientPZ0github.com/momentohq/client-sdk-go;client_sdk_go�Momento.Protos.CacheClientJ�
  �

  

 G
	
 G

 "
	

 "

 *
	
 *

 7
	
% 7

 


 	 


 	

  


  
	

  


 	

 

 

 


 

 	

 

 

 	



 

  

  

  


  J


 

  1

  	

  

  !-

 1

 	

 

 !-

 R

 

 ,

 7N

 :

 

 

 '6

 C

 

 "

 -?

 O

 

 *

 5K

 U

 

 .

 9Q

 O

 

 *

 5K

 a

 

 6

 A]

 	X

 	

 	0

 	;T

 
@

 


 
 

 
+<

  @

  

   

  +<

 !O

 !

 !*

 !5K

 #N

 #

 #)

 #4J

 $K

 $

 $'

 $2G

 %K

 %

 %'

 %2G

 &H

 &

 &%

 &0D

 'B

 '

 '!

 ',>

 (E

 (

 (#

 (.A

 )B

 )

 )!

 ),>

 *E

 *

 *#

 *.A

 +c

 +

 +7

 +B_

 ,`

 ,

 ,5

 ,@\

 -E

 -

 -#

 -.A
�
 8K� Add or Updates new element with its score to the Sorted Set.
 If sorted set doesn't exist, a new one is created with the specified
 element and its associated score.
 If an element exists, then its associate score gets overridden with the one
 provided in this operation.
2� Sorted Set Operations
 A sorted set is a collection of elements ordered by their score.
 The elements with same score are ordered lexicographically.


 8

 8'

 82G
>
 :Q1 Fetches a subset of elements in the sorted set.


 :

 :+

 :6M
c
 =ZV Gets the specified element and its associated score if it exists in the
 sorted set.


 =

 =1

 =<V
E
 ?T8 Removes specified elements and their associated scores


 ?

 ?-

 ?8P
�
 G]� Changes the score associated with the element by specified amount.
 If the provided amount is negative, then the score associated with the
 element is decremented.
 If the element that needs to be incremented isn't present in the sorted
 set, it is added with specified number as the score.
 If the set itself doesn't exist then a new one with specified element and
 score is created.


 G

 G3

 G>Y
,
 IW Gives the rank of an element.


 I

 I/

 I:S


 L N


 L

  M

  M

  M

  M


P T


P

 Q

 Q

 Q

 Q

R

R

R

R

S

S

S	

S


V X


V

 W

 W

 W

 W
	
Z 


Z


\ `


\

 ]

 ]

 ]

 ]

^

^

^

^

_

_

_	

_


b e


b

 c

 c

 c

 c

d

d

d	

d


g k


g

 h

 h

 h

 h

i

i

i

i

j

j

j	

j


m t


m

 nq

 n

 o

 o

 o

 o

p

p

p

p

 r

 r


s

s



v |


v

 w

 w

 w

 w
�
z� Amount to add to the stored value.
 If this key doesn't currently exist, it's created with this value (encoded as a base 10 string)


z

z

z

{

{

{	

{

	~ �


	~
?
	 �1 The value stored after the increment operation.


	 �

	 �

	 �


� �


�


 �


 �


 �


 �


�


�



�


�


�

� �

�

 ��

 �
$

  �

  �

  �

  �

 �

 �	

 �


 �

��

�


 �2

 �

 �'

 �(-

 �01

�

�


 ��

 �

 �

 �


 �

 �

�

�

�

�

� �

�

 �

 �

 �

 �

� �

�!

 �

 �

 �

 �

�

�

�

�

� �

� 

 ��

 �


  �1

  �

  �&

  �',

  �/0

�

�


 ��

 �

 �

 �


 �

 �

�

�

�

�

� �

�

 �

 �

 �

 �

�/

�


�$

�%*

�-.

�

�

�	

�

�

�

�

�


� !

�

� �

�#

 �

 �

 �

 �

�

�

�

�

�

�

�

�

�

�

�	

�

�

�

�

�

� �

�$

 �

 �

 �

 �

� �

� 

 ��

 �


  �

  �

  �

  �

  �

�

�


 �

 �

 �

 �

 ��

 �

�

�

�	

�

�

�

�

�


� $

�!

� �

�

 �

 �

 �

 �

� �

�

 ��

 �


  � 

  �

  �

  �

  �

�

�


 ��

 �

 �

 �


 �

 �

�

�

�

�

� �

�

 �

 �

 �

 �

�

�


�

�

�

�

�

�	

�

�

�

�

�


� 

�

� �

�
*
 �� cache = request - stored


 �


  � 

  �

  �

  �

  �
*
�� cache = stored - request


�

.
 �� Subtract a set of elements


 �

  �"

	  �

	  �

	  �

	  � !
V
�F Subtract the set's identity (itself) from itself - which deletes it.


�

 ��

 �


 �

 �


 �

 �

�

�

�

�

 �

 �

 �

 �

 ��

 �

�

�

�

�

�

�

�

�

� �

�

 �

 �


�

�


 ��

 �

 �

 �


 �

 �

�

�

�

�

� �

�$

 �

 �

 �

 �

�

�


�

�

�

�

�

�	

�

�

�

�

�
L
�#> ensure total length <= this; remove excess from back of list


�

�	

�!"

� �

�%
:
 �, length of the list after the concatenation


 �

 �	

 �

� �

�#

 �

 �

 �

 �

�

�


�

�

�

�

�

�	

�

�

�

�

�
M
�$? ensure total length <= this; remove excess from front of list


�

�	

�"#

� �

�$
:
 �, length of the list after the concatenation


 �

 �	

 �
)
� � stored = request + stored


�

 �

 �

 �

 �

�

�

�

�

�

�

�	

�

�

�

�

�
L
�#> ensure total length <= this; remove excess from back of list


�

�	

�!"

 � �

 �
1
  �# length of the list after the push


  �

  �	

  �
)
!� � stored = stored + request


!�

! �

! �

! �

! �

!�

!�

!�

!�

!�

!�

!�	

!�

!�

!�

!�

!�
M
!�$? ensure total length <= this; remove excess from front of list


!�

!�	

!�"#

"� �

"�
1
" �# length of the list after the push


" �

" �	

" �

#� �

#�

# �

# �

# �

# �

$� �

$�

$ ��

$ �


$  �

$  �	

$  �


$  �

$�

$�


$ ��

$ �

$ �

$ �


$ �

$ �

$�

$�

$�

$�

%� �

%�

% �

% �

% �

% �

&� �

&�

& ��

& �


&  �

&  �	

&  �


&  �

&�

&�


& ��

& �

& �

& �


& �

& �

&�

&�

&�

&�

'� �

'�

' �

' �

' �	

' �

'�

'�

'�	

'�

(� �

(�

( �

( �


(��

(�


( �#

( �

( �

( �

( �!"

( �

( �

( �

( �

( ��

( �

(�

(�

(�

(�

(�

(�

(�	

(�


)� 

)�

*� �

*�

* �

* �

* �

* �

* ��

* �
R
*�&D Remove all appearances in the list where the element is this value


*�	

*�
!

*�$%


+� 

+�

,� �

,�

, �

, �

, �

, �

,�

,�

,�	

,�

,�

,�

,�	

,�

-� �

-�

- ��

- �


-  �

-  �

-  �

-  �

-  �

-�

-�


- ��

- �

- �

- �


- �

- �

-�

-�

-�

-�

.� �

.�

. �

. �

. �

. �

/� �

/�

/ ��

/ �


/  �

/  �


/  �

/  �

/�

/�


/ ��

/ �

/ �

/ �


/ �

/ �

/�

/�

/�

/�

0� �

0�

0 �

0 �

0 �

0 �

0�

0�

0�	

0�

0�

0�

0�	

0�

0�

0�

0�	

0�

0�

0�

0�

0�


1� 

1�

2� �

2�

2 �

2 �

2 �

2 �

2�

2�

2�	

2�

3� �

3�

3 �

3 �

3 �

3 �

3�*

3�


3�

3�%

3�()

3�

3�

3�	

3�

3�

3�

3�

3�


4�  

4�

5� �

5�

5 ��

5 �

5  �

5  �

5  �

5 �

5 �

5 �
"
5 � num_results fields


5 �


5��

5�


5 �

5 �


5 �

5 �

5� range fields


5�


5��

5�


5 ��

5 �

5  ��

	5  �


5   �

5   �

5   �

5   �
Z

5  �"F if true, the endpoint is open (exclusive); false, closed (inclusive)


5  �

5  �

5  �

5 �

	5 �

5  ��

	5  �

5  �

	5  �

	5  �

	5  �

5 �

	5 �

	5 �

	5 �

5 �

5 �

5 �

5 �

5�

5�

5�

5�

5 �

5 �

5 �

5 �

5�

5�

5�

5�

5 ��

5 �

5�

5�

5�	

5�

5�

5�


5�

5�

5��

5�

5�

5�

5�

5�

5�

5�

5�

5�

6� �

6�

6 ��

6 �


6  �,

6  �

6  �

6  �'

6  �*+

6��

6�


6 ��

6 �

6 �

6 �


6 �

6 �

6�

6�

6�

6�

7� �

7�!

7 �

7 �

7 �

7 �

7�"

7�


7�

7�

7� !

8� �

8�"

8 ��

8 �
(

8  �

8  �

8  �

8  �

8 �

8 �


8 �

8 �

8��

8�


8 �9

8 �

8 �+

8 �,4

8 �78

8��

8�


8 ��

8 �

8 �

8 �

8 �

8 �

8�"

8�

8�

8� !

9� �

9�

9 �

9 �

9 �

9 �

9 �

9 �


9��

9�


9 �$

9 �

9 �

9 �

9 �"#

9 ��

9 �

9�

9�

9�	

9�

9�

9�	

9�


9�


:� #

:� 

;� �

;�"

; �

; �

; �

; �

;�

;�

;�

;�

;�

;�

;�	

;�

;�

;�

;�	

;�

;�

;�

;�

;�

<� �

<�#
?
< �1 The value stored after the increment operation.


< �

< �	

< �

=� �

=� 

= �

= �

= �

= �

=�

=�

=�

=�

>� �

>�!

> ��

> �


>  �

>  �

>  �

>  �

> �

> �


> �

> �

>�

>�


> ��

> �

> �'

> �

> �"

> �%&

>�"

>�

>�

>� !bproto3
�
cacheping.protocache_client"
_PingRequest"
_PingResponse2I
PingA
Ping.cache_client._PingRequest.cache_client._PingResponse" B0
grpc.cache_clientP�Momento.Protos.CachePingJ�
  

  

 "
	

 "

 *
	
 *

 5
	
% 5

 


  



 

  	4

  	


  	

  	#0
	
  


 
	
 


bproto3
�9
cachepubsub.protocache_client.pubsub"
_Empty"~
_PublishRequest

cache_name (	R	cacheName
topic (	Rtopic6
value (2 .cache_client.pubsub._TopicValueRvalue"�
_SubscriptionRequest

cache_name (	R	cacheName
topic (	RtopicD
resume_at_topic_sequence_number (RresumeAtTopicSequenceNumber"�
_SubscriptionItem5
item (2.cache_client.pubsub._TopicItemH RitemK
discontinuity (2#.cache_client.pubsub._DiscontinuityH Rdiscontinuity?
	heartbeat (2.cache_client.pubsub._HeartbeatH R	heartbeatB
kind"x

_TopicItem2
topic_sequence_number (RtopicSequenceNumber6
value (2 .cache_client.pubsub._TopicValueRvalue"E
_TopicValue
text (	H Rtext
binary (H RbinaryB
kind"n
_Discontinuity.
last_topic_sequence (RlastTopicSequence,
new_topic_sequence (RnewTopicSequence"

_Heartbeat2�
PubsubL
Publish$.cache_client.pubsub._PublishRequest.cache_client.pubsub._Empty`
	Subscribe).cache_client.pubsub._SubscriptionRequest&.cache_client.pubsub._SubscriptionItem0Br
grpc.cache_client.pubsubPZ0github.com/momentohq/client-sdk-go;client_sdk_go�!Momento.Protos.CacheClient.PubsubJ�/
  �

  

 G
	
 G

 "
	

 "

 1
	
 1

 >
	
% >

 
�
  +� For working with topics in a cache.
 Momento topics are conceptually located on a cache. They are best-effort multicast.
 To use them, create a cache then start subscribing and publishing!

 Momento topic subscriptions try to give you information about the quality of the
   stream you are receiving. For example, you might miss messages if your network
   is slow, or if some intermediate switch fails, or due to rate limiting. It is
   also possible, though we try to avoid it, that messages could briefly come out
   of order between subscribers.
   We try to tell you when things like this happen via a Discontinuity in your
   subscription stream. If you do not care about occasional discontinuities then
   don't bother handling them! You might still want to log them just in case ;-)



 
�
  "0� Publish a message to a topic.

 If a topic has no subscribers, then the effect of Publish MAY be either of:
 * It is dropped and the topic is nonexistent.
 * It is accepted to the topic as the next message.

 Publish() does not wait for subscribers to accept. It returns Ok upon accepting
 the topic value. It also returns Ok if there are no subscribers and the value
 happens to be dropped. Publish() can not guarantee delivery in theory but in
 practice it should almost always deliver to subscribers.

 REQUIRES HEADER authorization: Momento auth token


  "

  "

  "(.
�
 *I� Subscribe to notifications from a topic.

 You will receive a stream of values and (hopefully occasional) discontinuities.
 Values will appear as copies of the payloads you Publish() to the topic.

 REQUIRES HEADER authorization: Momento auth token


 *

 *$

 */5

 *6G
	
 - 


 -
1
0 8% A value to publish through a topic.



0
Q
 2D Cache namespace for the topic to which you want to send the value.


 2

 2	

 2
J
4= The literal topic name to which you want to send the value.


4

4	

4
�
7� The value you want to send to the topic. All current subscribers will receive
 this, should the whims of the Internet prove merciful.


7

7

7
D
; I8 A description of how you want to subscribe to a topic.



;
L
 =? Cache namespace for the topic to which you want to subscribe.


 =

 =	

 =
E
@8 The literal topic name to which you want to subscribe.


@

@	

@
�
H-� --> Providing this is not required. <--

 If provided, attempt to reconnect to the topic and replay messages starting from
 the provided sequence number. You may get a discontinuity if some (or all) of the
 messages are not available.
 If not provided (or 0), the subscription will begin with the latest messages.


H

H	(

H+,
�
N ]� Possible message kinds from a topic. They can be items when they're from you, or
 other kinds when we have something we think you might need to know about the
 subscription's status.



N

 O\

 O
M
 Q@ The subscription has yielded an item you previously published.


 Q

 Q

 Q
�
X%� Momento wants to let you know we detected some possible inconsistency at this
 point in the subscription stream.

 A lack of a discontinuity does not mean the subscription is guaranteed to be
 strictly perfect, but the presence of a discontinuity is very likely to


X

X 

X#$
H
[; The stream is still working, there's nothing to see here.


[

[

[
Y
` lM Your subscription has yielded an item you previously published. Here it is!



`
�
 i#� Topic sequence numbers are **best-effort** and **informational**.
 They are not transactional.
 They exist:
 * to help reconnect to an existing topic while trying to avoid missing items.
 * to facilitate richer monitoring and logging.
 * to provide a best-effort awareness of stream contiguity, or lack thereof,
   in case you need to know.
 You can safely ignore them if none of that matters to you!


 i

 i	

 i!"
@
k3 The value you previously published to this topic.


k

k

k
X
o wL A value in a topic - published, duplicated and received in a subscription.



o
�
 sv� Types of messages a topic may relay. You can mix types or you can make conventionally
 typed topics. Sticking with one kind will generally make your software easier to work
 with though, so we recommend picking the kind you like and using it for a topic!


 s

 t

 t


 t

 t

u

u	

u


u
�
| �� A message from Momento when we know a subscription to have skipped some messages.
 We don't terminate your subscription in that case, but just in case you care, we
 do our best to let you know about it.



|
i
 ~!\ The last topic value sequence number known to have been attempted (if known, 0 otherwise).


 ~

 ~	

 ~ 
\
� N The new topic sequence number after which TopicItems will ostensibly resume.


�

�	

�
�
� �� A message from Momento for when we want to reassure clients or frameworks that a
 Subscription is still healthy.
 These are synthetic meta-events and do not increase the topic sequence count.
 Different subscribers may receive a different cadence of heartbeat, and no guarantee
 is made about the cadence or even presence or absence of heartbeats in a stream.
 They are a tool for helping ensure that socket timeouts and the like don't impact
 subscriptions you may care about, but that aren't receiving a substantial publish rate.


�bproto3
�
controlclient.protocontrol_client"4
_DeleteCacheRequest

cache_name (	R	cacheName"
_DeleteCacheResponse"4
_CreateCacheRequest

cache_name (	R	cacheName"
_CreateCacheResponse"3
_ListCachesRequest

next_token (	R	nextToken"'
_Cache

cache_name (	R	cacheName"b
_ListCachesResponse,
cache (2.control_client._CacheRcache

next_token (	R	nextToken";
_CreateSigningKeyRequest
ttl_minutes (R
ttlMinutes"L
_CreateSigningKeyResponse
key (	Rkey

expires_at (R	expiresAt"1
_RevokeSigningKeyRequest
key_id (	RkeyId"
_RevokeSigningKeyResponse"C
_SigningKey
key_id (	RkeyId

expires_at (R	expiresAt"8
_ListSigningKeysRequest

next_token (	R	nextToken"w
_ListSigningKeysResponse<
signing_key (2.control_client._SigningKeyR
signingKey

next_token (	R	nextToken"3
_FlushCacheRequest

cache_name (	R	cacheName"
_FlushCacheResponse2�

ScsControlZ
CreateCache#.control_client._CreateCacheRequest$.control_client._CreateCacheResponse" Z
DeleteCache#.control_client._DeleteCacheRequest$.control_client._DeleteCacheResponse" W

ListCaches".control_client._ListCachesRequest#.control_client._ListCachesResponse" W

FlushCache".control_client._FlushCacheRequest#.control_client._FlushCacheResponse" i
CreateSigningKey(.control_client._CreateSigningKeyRequest).control_client._CreateSigningKeyResponse" i
RevokeSigningKey(.control_client._RevokeSigningKeyRequest).control_client._RevokeSigningKeyResponse" f
ListSigningKeys'.control_client._ListSigningKeysRequest(.control_client._ListSigningKeysResponse" Bh
grpc.control_clientPZ0github.com/momentohq/client-sdk-go;client_sdk_go�Momento.Protos.ControlClientJ�
  T

  

 G
	
 G

 "
	

 "

 ,
	
 ,

 9
	
% 9

 


 	 


 	

  
I

  


  
&

  
1E

 I

 

 &

 1E

 F

 

 $

 /B

 F

 

 $

 /B

 X

 

 0

 ;T

 X

 

 0

 ;T

 U

 

 .

 9Q


  


 

  

  

  	

  


 





 




 

 

 	

 


 





! #


!

 "

 "

 "	

 "


% '


%

 &

 &

 &	

 &


) ,


)

 *

 *


 *

 *

 *

+

+

+	

+


. 0


. 

 /

 /

 /	

 /


2 5


2!

 3

 3

 3	

 3

4

4

4	

4


	7 9


	7 

	 8

	 8

	 8	

	 8



; <



;!


> C


>
(
 @ The id of the signing key


 @

 @	

 @
A
B4 Epoch time in seconds when the signing key expires


B

B	

B


E G


E

 F

 F

 F	

 F


I L


I 

 J'

 J


 J

 J"

 J%&

K

K

K	

K


N P


N

 O

 O

 O	

 O


R T


Rbproto3
�x
google/api/http.proto
google.api"y
Http*
rules (2.google.api.HttpRuleRrulesE
fully_decode_reserved_expansion (RfullyDecodeReservedExpansion"�
HttpRule
selector (	Rselector
get (	H Rget
put (	H Rput
post (	H Rpost
delete (	H Rdelete
patch (	H Rpatch7
custom (2.google.api.CustomHttpPatternH Rcustom
body (	Rbody#
response_body (	RresponseBodyE
additional_bindings (2.google.api.HttpRuleRadditionalBindingsB	
pattern";
CustomHttpPattern
kind (	Rkind
path (	RpathBj
com.google.apiB	HttpProtoPZAgoogle.golang.org/genproto/googleapis/api/annotations;annotations��GAPIJ�s
 �
�
 2� Copyright 2015 Google LLC

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.


 

 
	
 

 X
	
 X

 "
	

 "

 *
	
 *

 '
	
 '

 "
	
$ "
�
  )� Defines the HTTP configuration for an API service. It contains a list of
 [HttpRule][google.api.HttpRule], each specifying the mapping of an RPC method
 to one or more HTTP REST API methods.



 
�
   � A list of HTTP configuration rules that apply to individual API methods.

 **NOTE:** All service configuration rules follow "last one wins" order.


   


   

   

   
�
 (+� When set to true, URL path parameters will be fully URI-decoded except in
 cases of single segment matches in reserved expansion, where "%2F" will be
 left encoded.

 The default behavior is to not decode RFC 6570 reserved characters in multi
 segment matches.


 (

 (&

 ()*
�S
� ��S # gRPC Transcoding

 gRPC Transcoding is a feature for mapping between a gRPC method and one or
 more HTTP REST endpoints. It allows developers to build a single API service
 that supports both gRPC APIs and REST APIs. Many systems, including [Google
 APIs](https://github.com/googleapis/googleapis),
 [Cloud Endpoints](https://cloud.google.com/endpoints), [gRPC
 Gateway](https://github.com/grpc-ecosystem/grpc-gateway),
 and [Envoy](https://github.com/envoyproxy/envoy) proxy support this feature
 and use it for large scale production services.

 `HttpRule` defines the schema of the gRPC/REST mapping. The mapping specifies
 how different portions of the gRPC request message are mapped to the URL
 path, URL query parameters, and HTTP request body. It also controls how the
 gRPC response message is mapped to the HTTP response body. `HttpRule` is
 typically specified as an `google.api.http` annotation on the gRPC method.

 Each mapping specifies a URL path template and an HTTP method. The path
 template may refer to one or more fields in the gRPC request message, as long
 as each field is a non-repeated field with a primitive (non-message) type.
 The path template controls how fields of the request message are mapped to
 the URL path.

 Example:

     service Messaging {
       rpc GetMessage(GetMessageRequest) returns (Message) {
         option (google.api.http) = {
             get: "/v1/{name=messages/*}"
         };
       }
     }
     message GetMessageRequest {
       string name = 1; // Mapped to URL path.
     }
     message Message {
       string text = 1; // The resource content.
     }

 This enables an HTTP REST to gRPC mapping as below:

 HTTP | gRPC
 -----|-----
 `GET /v1/messages/123456`  | `GetMessage(name: "messages/123456")`

 Any fields in the request message which are not bound by the path template
 automatically become HTTP query parameters if there is no HTTP request body.
 For example:

     service Messaging {
       rpc GetMessage(GetMessageRequest) returns (Message) {
         option (google.api.http) = {
             get:"/v1/messages/{message_id}"
         };
       }
     }
     message GetMessageRequest {
       message SubMessage {
         string subfield = 1;
       }
       string message_id = 1; // Mapped to URL path.
       int64 revision = 2;    // Mapped to URL query parameter `revision`.
       SubMessage sub = 3;    // Mapped to URL query parameter `sub.subfield`.
     }

 This enables a HTTP JSON to RPC mapping as below:

 HTTP | gRPC
 -----|-----
 `GET /v1/messages/123456?revision=2&sub.subfield=foo` |
 `GetMessage(message_id: "123456" revision: 2 sub: SubMessage(subfield:
 "foo"))`

 Note that fields which are mapped to URL query parameters must have a
 primitive type or a repeated primitive type or a non-repeated message type.
 In the case of a repeated type, the parameter can be repeated in the URL
 as `...?param=A&param=B`. In the case of a message type, each field of the
 message is mapped to a separate parameter, such as
 `...?foo.a=A&foo.b=B&foo.c=C`.

 For HTTP methods that allow a request body, the `body` field
 specifies the mapping. Consider a REST update method on the
 message resource collection:

     service Messaging {
       rpc UpdateMessage(UpdateMessageRequest) returns (Message) {
         option (google.api.http) = {
           patch: "/v1/messages/{message_id}"
           body: "message"
         };
       }
     }
     message UpdateMessageRequest {
       string message_id = 1; // mapped to the URL
       Message message = 2;   // mapped to the body
     }

 The following HTTP JSON to RPC mapping is enabled, where the
 representation of the JSON in the request body is determined by
 protos JSON encoding:

 HTTP | gRPC
 -----|-----
 `PATCH /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id:
 "123456" message { text: "Hi!" })`

 The special name `*` can be used in the body mapping to define that
 every field not bound by the path template should be mapped to the
 request body.  This enables the following alternative definition of
 the update method:

     service Messaging {
       rpc UpdateMessage(Message) returns (Message) {
         option (google.api.http) = {
           patch: "/v1/messages/{message_id}"
           body: "*"
         };
       }
     }
     message Message {
       string message_id = 1;
       string text = 2;
     }


 The following HTTP JSON to RPC mapping is enabled:

 HTTP | gRPC
 -----|-----
 `PATCH /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id:
 "123456" text: "Hi!")`

 Note that when using `*` in the body mapping, it is not possible to
 have HTTP parameters, as all fields not bound by the path end in
 the body. This makes this option more rarely used in practice when
 defining REST APIs. The common usage of `*` is in custom methods
 which don't use the URL at all for transferring data.

 It is possible to define multiple HTTP methods for one RPC by using
 the `additional_bindings` option. Example:

     service Messaging {
       rpc GetMessage(GetMessageRequest) returns (Message) {
         option (google.api.http) = {
           get: "/v1/messages/{message_id}"
           additional_bindings {
             get: "/v1/users/{user_id}/messages/{message_id}"
           }
         };
       }
     }
     message GetMessageRequest {
       string message_id = 1;
       string user_id = 2;
     }

 This enables the following two alternative HTTP JSON to RPC mappings:

 HTTP | gRPC
 -----|-----
 `GET /v1/messages/123456` | `GetMessage(message_id: "123456")`
 `GET /v1/users/me/messages/123456` | `GetMessage(user_id: "me" message_id:
 "123456")`

 ## Rules for HTTP mapping

 1. Leaf request fields (recursive expansion nested messages in the request
    message) are classified into three categories:
    - Fields referred by the path template. They are passed via the URL path.
    - Fields referred by the [HttpRule.body][google.api.HttpRule.body]. They are passed via the HTTP
      request body.
    - All other fields are passed via the URL query parameters, and the
      parameter name is the field path in the request message. A repeated
      field can be represented as multiple query parameters under the same
      name.
  2. If [HttpRule.body][google.api.HttpRule.body] is "*", there is no URL query parameter, all fields
     are passed via URL path and HTTP request body.
  3. If [HttpRule.body][google.api.HttpRule.body] is omitted, there is no HTTP request body, all
     fields are passed via URL path and URL query parameters.

 ### Path template syntax

     Template = "/" Segments [ Verb ] ;
     Segments = Segment { "/" Segment } ;
     Segment  = "*" | "**" | LITERAL | Variable ;
     Variable = "{" FieldPath [ "=" Segments ] "}" ;
     FieldPath = IDENT { "." IDENT } ;
     Verb     = ":" LITERAL ;

 The syntax `*` matches a single URL path segment. The syntax `**` matches
 zero or more URL path segments, which must be the last part of the URL path
 except the `Verb`.

 The syntax `Variable` matches part of the URL path as specified by its
 template. A variable template must not contain other variables. If a variable
 matches a single path segment, its template may be omitted, e.g. `{var}`
 is equivalent to `{var=*}`.

 The syntax `LITERAL` matches literal text in the URL path. If the `LITERAL`
 contains any reserved character, such characters should be percent-encoded
 before the matching.

 If a variable contains exactly one path segment, such as `"{var}"` or
 `"{var=*}"`, when such a variable is expanded into a URL path on the client
 side, all characters except `[-_.~0-9a-zA-Z]` are percent-encoded. The
 server side does the reverse decoding. Such variables show up in the
 [Discovery
 Document](https://developers.google.com/discovery/v1/reference/apis) as
 `{var}`.

 If a variable contains multiple path segments, such as `"{var=foo/*}"`
 or `"{var=**}"`, when such a variable is expanded into a URL path on the
 client side, all characters except `[-_.~/0-9a-zA-Z]` are percent-encoded.
 The server side does the reverse decoding, except "%2F" and "%2f" are left
 unchanged. Such variables show up in the
 [Discovery
 Document](https://developers.google.com/discovery/v1/reference/apis) as
 `{+var}`.

 ## Using gRPC API Service Configuration

 gRPC API Service Configuration (service config) is a configuration language
 for configuring a gRPC service to become a user-facing product. The
 service config is simply the YAML representation of the `google.api.Service`
 proto message.

 As an alternative to annotating your proto file, you can configure gRPC
 transcoding in your service config YAML files. You do this by specifying a
 `HttpRule` that maps the gRPC method to a REST endpoint, achieving the same
 effect as the proto annotation. This can be particularly useful if you
 have a proto that is reused in multiple services. Note that any transcoding
 specified in the service config will override any matching transcoding
 configuration in the proto.

 Example:

     http:
       rules:
         # Selects a gRPC method and applies HttpRule to it.
         - selector: example.v1.Messaging.GetMessage
           get: /v1/messages/{message_id}/{sub.subfield}

 ## Special notes

 When gRPC Transcoding is used to map a gRPC to JSON REST endpoints, the
 proto to JSON conversion must follow the [proto3
 specification](https://developers.google.com/protocol-buffers/docs/proto3#json).

 While the single segment variable follows the semantics of
 [RFC 6570](https://tools.ietf.org/html/rfc6570) Section 3.2.2 Simple String
 Expansion, the multi segment variable **does not** follow RFC 6570 Section
 3.2.3 Reserved Expansion. The reason is that the Reserved Expansion
 does not expand special characters like `?` and `#`, which would lead
 to invalid URLs. As the result, gRPC Transcoding uses a custom encoding
 for multi segment variables.

 The path variables **must not** refer to any repeated or mapped field,
 because client libraries are not capable of handling such variable expansion.

 The path variables **must not** capture the leading "/" character. The reason
 is that the most common use case "{var}" does not capture the leading "/"
 character. For consistency, all path variables must share the same behavior.

 Repeated message fields must not be mapped to URL query parameters, because
 no client library can support such complicated mapping.

 If an API needs to use a JSON array for request or response body, it can map
 the request or response body to a repeated field. However, some gRPC
 Transcoding implementations may not support this feature.


�
�
 � Selects a method to which this rule applies.

 Refer to [selector][google.api.DocumentationRule.selector] for syntax details.


 �

 �	

 �
�
 ��� Determines the URL pattern is matched by this rules. This pattern can be
 used with any of the {get|put|post|delete|patch} methods. A custom method
 can be defined using the 'custom' field.


 �
\
�N Maps to HTTP GET. Used for listing and getting information about
 resources.


�


�

�
@
�2 Maps to HTTP PUT. Used for replacing a resource.


�


�

�
X
�J Maps to HTTP POST. Used for creating a resource or performing an action.


�


�

�
B
�4 Maps to HTTP DELETE. Used for deleting a resource.


�


�

�
A
�3 Maps to HTTP PATCH. Used for updating a resource.


�


�

�
�
�!� The custom pattern is used for specifying an HTTP method that is not
 included in the `pattern` field, such as HEAD, or "*" to leave the
 HTTP method unspecified for this rule. The wild-card rule is useful
 for services that provide content to Web (HTML) clients.


�

�

� 
�
�� The name of the request field whose value is mapped to the HTTP request
 body, or `*` for mapping all request fields not captured by the path
 pattern to the HTTP body, or omitted for not having any HTTP request body.

 NOTE: the referred field must be present at the top-level of the request
 message type.


�

�	

�
�
�� Optional. The name of the response field whose value is mapped to the HTTP
 response body. When omitted, the entire response message will be used
 as the HTTP response body.

 NOTE: The referred field must be present at the top-level of the response
 message type.


�

�	

�
�
	�-� Additional HTTP bindings for the selector. Nested bindings must
 not contain an `additional_bindings` field themselves (that is,
 the nesting may only be one level deep).


	�


	�

	�'

	�*,
G
� �9 A custom pattern is used for defining custom HTTP verb.


�
2
 �$ The name of this custom HTTP verb.


 �

 �	

 �
5
�' The path matched by this custom verb.


�

�	

�bproto3
Ʌ
 google/protobuf/descriptor.protogoogle.protobuf"M
FileDescriptorSet8
file (2$.google.protobuf.FileDescriptorProtoRfile"�
FileDescriptorProto
name (	Rname
package (	Rpackage

dependency (	R
dependency+
public_dependency
 (RpublicDependency'
weak_dependency (RweakDependencyC
message_type (2 .google.protobuf.DescriptorProtoRmessageTypeA
	enum_type (2$.google.protobuf.EnumDescriptorProtoRenumTypeA
service (2'.google.protobuf.ServiceDescriptorProtoRserviceC
	extension (2%.google.protobuf.FieldDescriptorProtoR	extension6
options (2.google.protobuf.FileOptionsRoptionsI
source_code_info	 (2.google.protobuf.SourceCodeInfoRsourceCodeInfo
syntax (	Rsyntax"�
DescriptorProto
name (	Rname;
field (2%.google.protobuf.FieldDescriptorProtoRfieldC
	extension (2%.google.protobuf.FieldDescriptorProtoR	extensionA
nested_type (2 .google.protobuf.DescriptorProtoR
nestedTypeA
	enum_type (2$.google.protobuf.EnumDescriptorProtoRenumTypeX
extension_range (2/.google.protobuf.DescriptorProto.ExtensionRangeRextensionRangeD

oneof_decl (2%.google.protobuf.OneofDescriptorProtoR	oneofDecl9
options (2.google.protobuf.MessageOptionsRoptionsU
reserved_range	 (2..google.protobuf.DescriptorProto.ReservedRangeRreservedRange#
reserved_name
 (	RreservedNamez
ExtensionRange
start (Rstart
end (Rend@
options (2&.google.protobuf.ExtensionRangeOptionsRoptions7
ReservedRange
start (Rstart
end (Rend"|
ExtensionRangeOptionsX
uninterpreted_option� (2$.google.protobuf.UninterpretedOptionRuninterpretedOption*	�����"�
FieldDescriptorProto
name (	Rname
number (RnumberA
label (2+.google.protobuf.FieldDescriptorProto.LabelRlabel>
type (2*.google.protobuf.FieldDescriptorProto.TypeRtype
	type_name (	RtypeName
extendee (	Rextendee#
default_value (	RdefaultValue
oneof_index	 (R
oneofIndex
	json_name
 (	RjsonName7
options (2.google.protobuf.FieldOptionsRoptions'
proto3_optional (Rproto3Optional"�
Type
TYPE_DOUBLE

TYPE_FLOAT

TYPE_INT64
TYPE_UINT64

TYPE_INT32
TYPE_FIXED64
TYPE_FIXED32
	TYPE_BOOL
TYPE_STRING	

TYPE_GROUP

TYPE_MESSAGE

TYPE_BYTES
TYPE_UINT32
	TYPE_ENUM
TYPE_SFIXED32
TYPE_SFIXED64
TYPE_SINT32
TYPE_SINT64"C
Label
LABEL_OPTIONAL
LABEL_REQUIRED
LABEL_REPEATED"c
OneofDescriptorProto
name (	Rname7
options (2.google.protobuf.OneofOptionsRoptions"�
EnumDescriptorProto
name (	Rname?
value (2).google.protobuf.EnumValueDescriptorProtoRvalue6
options (2.google.protobuf.EnumOptionsRoptions]
reserved_range (26.google.protobuf.EnumDescriptorProto.EnumReservedRangeRreservedRange#
reserved_name (	RreservedName;
EnumReservedRange
start (Rstart
end (Rend"�
EnumValueDescriptorProto
name (	Rname
number (Rnumber;
options (2!.google.protobuf.EnumValueOptionsRoptions"�
ServiceDescriptorProto
name (	Rname>
method (2&.google.protobuf.MethodDescriptorProtoRmethod9
options (2.google.protobuf.ServiceOptionsRoptions"�
MethodDescriptorProto
name (	Rname

input_type (	R	inputType
output_type (	R
outputType8
options (2.google.protobuf.MethodOptionsRoptions0
client_streaming (:falseRclientStreaming0
server_streaming (:falseRserverStreaming"�	
FileOptions!
java_package (	RjavaPackage0
java_outer_classname (	RjavaOuterClassname5
java_multiple_files
 (:falseRjavaMultipleFilesD
java_generate_equals_and_hash (BRjavaGenerateEqualsAndHash:
java_string_check_utf8 (:falseRjavaStringCheckUtf8S
optimize_for	 (2).google.protobuf.FileOptions.OptimizeMode:SPEEDRoptimizeFor

go_package (	R	goPackage5
cc_generic_services (:falseRccGenericServices9
java_generic_services (:falseRjavaGenericServices5
py_generic_services (:falseRpyGenericServices7
php_generic_services* (:falseRphpGenericServices%

deprecated (:falseR
deprecated.
cc_enable_arenas (:trueRccEnableArenas*
objc_class_prefix$ (	RobjcClassPrefix)
csharp_namespace% (	RcsharpNamespace!
swift_prefix' (	RswiftPrefix(
php_class_prefix( (	RphpClassPrefix#
php_namespace) (	RphpNamespace4
php_metadata_namespace, (	RphpMetadataNamespace!
ruby_package- (	RrubyPackageX
uninterpreted_option� (2$.google.protobuf.UninterpretedOptionRuninterpretedOption":
OptimizeMode	
SPEED
	CODE_SIZE
LITE_RUNTIME*	�����J&'"�
MessageOptions<
message_set_wire_format (:falseRmessageSetWireFormatL
no_standard_descriptor_accessor (:falseRnoStandardDescriptorAccessor%

deprecated (:falseR
deprecated
	map_entry (RmapEntryX
uninterpreted_option� (2$.google.protobuf.UninterpretedOptionRuninterpretedOption*	�����JJJJ	J	
"�
FieldOptionsA
ctype (2#.google.protobuf.FieldOptions.CType:STRINGRctype
packed (RpackedG
jstype (2$.google.protobuf.FieldOptions.JSType:	JS_NORMALRjstype
lazy (:falseRlazy%

deprecated (:falseR
deprecated
weak
 (:falseRweakX
uninterpreted_option� (2$.google.protobuf.UninterpretedOptionRuninterpretedOption"/
CType

STRING 
CORD
STRING_PIECE"5
JSType
	JS_NORMAL 
	JS_STRING
	JS_NUMBER*	�����J"s
OneofOptionsX
uninterpreted_option� (2$.google.protobuf.UninterpretedOptionRuninterpretedOption*	�����"�
EnumOptions
allow_alias (R
allowAlias%

deprecated (:falseR
deprecatedX
uninterpreted_option� (2$.google.protobuf.UninterpretedOptionRuninterpretedOption*	�����J"�
EnumValueOptions%

deprecated (:falseR
deprecatedX
uninterpreted_option� (2$.google.protobuf.UninterpretedOptionRuninterpretedOption*	�����"�
ServiceOptions%

deprecated! (:falseR
deprecatedX
uninterpreted_option� (2$.google.protobuf.UninterpretedOptionRuninterpretedOption*	�����"�
MethodOptions%

deprecated! (:falseR
deprecatedq
idempotency_level" (2/.google.protobuf.MethodOptions.IdempotencyLevel:IDEMPOTENCY_UNKNOWNRidempotencyLevelX
uninterpreted_option� (2$.google.protobuf.UninterpretedOptionRuninterpretedOption"P
IdempotencyLevel
IDEMPOTENCY_UNKNOWN 
NO_SIDE_EFFECTS

IDEMPOTENT*	�����"�
UninterpretedOptionA
name (2-.google.protobuf.UninterpretedOption.NamePartRname)
identifier_value (	RidentifierValue,
positive_int_value (RpositiveIntValue,
negative_int_value (RnegativeIntValue!
double_value (RdoubleValue!
string_value (RstringValue'
aggregate_value (	RaggregateValueJ
NamePart
	name_part (	RnamePart!
is_extension (RisExtension"�
SourceCodeInfoD
location (2(.google.protobuf.SourceCodeInfo.LocationRlocation�
Location
path (BRpath
span (BRspan)
leading_comments (	RleadingComments+
trailing_comments (	RtrailingComments:
leading_detached_comments (	RleadingDetachedComments"�
GeneratedCodeInfoM

annotation (2-.google.protobuf.GeneratedCodeInfo.AnnotationR
annotationm

Annotation
path (BRpath
source_file (	R
sourceFile
begin (Rbegin
end (RendB~
com.google.protobufBDescriptorProtosHZ-google.golang.org/protobuf/types/descriptorpb��GPB�Google.Protobuf.ReflectionJ��
' �
�
' 2� Protocol Buffers - Google's data interchange format
 Copyright 2008 Google Inc.  All rights reserved.
 https://developers.google.com/protocol-buffers/

 Redistribution and use in source and binary forms, with or without
 modification, are permitted provided that the following conditions are
 met:

     * Redistributions of source code must retain the above copyright
 notice, this list of conditions and the following disclaimer.
     * Redistributions in binary form must reproduce the above
 copyright notice, this list of conditions and the following disclaimer
 in the documentation and/or other materials provided with the
 distribution.
     * Neither the name of Google Inc. nor the names of its
 contributors may be used to endorse or promote products derived from
 this software without specific prior written permission.

 THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
2� Author: kenton@google.com (Kenton Varda)
  Based on original Protocol Buffers design by
  Sanjay Ghemawat, Jeff Dean, and others.

 The messages in this file describe the definitions found in .proto files.
 A valid .proto file can be translated directly to a FileDescriptorProto
 without any other information (e.g. without reading its imports).


) 

+ D
	
+ D

, ,
	
, ,

- 1
	
- 1

. 7
	
%. 7

/ !
	
$/ !

0 
	
0 

4 

	4 t descriptor.proto must be optimized for speed because reflection-based
 algorithms don't work during bootstrapping.

j
 8 :^ The protocol compiler can output a FileDescriptorSet containing the .proto
 files it parses.



 8

  9(

  9


  9

  9#

  9&'
/
= Z# Describes a complete .proto file.



=
9
 >", file name, relative to root of source tree


 >


 >

 >

 >
*
?" e.g. "foo", "foo.bar", etc.


?


?

?

?
4
B!' Names of files imported by this file.


B


B

B

B 
Q
D(D Indexes of the public imported files in the dependency list above.


D


D

D"

D%'
z
G&m Indexes of the weak imported files in the dependency list.
 For Google-internal migration only. Do not use.


G


G

G 

G#%
6
J,) All top-level definitions in this file.


J


J

J'

J*+

K-

K


K

K(

K+,

L.

L


L!

L")

L,-

M.

M


M

M )

M,-

	O#

	O


	O

	O

	O!"
�

U/� This field contains optional information about the original source code.
 You may safely remove this entire field without harming runtime
 functionality of the descriptors -- the information is needed only by
 development tools.



U



U


U*


U-.
]
YP The syntax of the proto file.
 The supported values are "proto2" and "proto3".


Y


Y

Y

Y
'
] } Describes a message type.



]

 ^

 ^


 ^

 ^

 ^

`*

`


`

` %

`()

a.

a


a

a )

a,-

c+

c


c

c&

c)*

d-

d


d

d(

d+,

 fk

 f


  g" Inclusive.


  g

  g

  g

  g

 h" Exclusive.


 h

 h

 h

 h

 j/

 j

 j"

 j#*

 j-.

l.

l


l

l)

l,-

n/

n


n

n *

n-.

p&

p


p

p!

p$%
�
ux� Range of reserved tag numbers. Reserved tag numbers may not be used by
 fields or extension ranges in the same message. Reserved ranges may
 not overlap.


u


 v" Inclusive.


 v

 v

 v

 v

w" Exclusive.


w

w

w

w

y,

y


y

y'

y*+
�
	|%u Reserved field names, which may not be used by fields in the same message.
 A given name may only be reserved once.


	|


	|

	|

	|"$

 �



O
 �:A The parser stores options it doesn't recognize here. See above.


 �


 �

 �3

 �69
Z
�M Clients can define custom options in extensions of this message. See above.


 �

 �

 �
3
� �% Describes a field within a message.


�

 ��

 �
S
  �C 0 is reserved for errors.
 Order is weird for historical reasons.


  �

  �

 �

 �

 �
w
 �g Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
 negative values are likely.


 �

 �

 �

 �

 �
w
 �g Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
 negative values are likely.


 �

 �

 �

 �

 �

 �

 �

 �

 �

 �

 �

 �

 �

 �
�
 	�� Tag-delimited aggregate.
 Group type is deprecated and not supported in proto3. However, Proto3
 implementations should still be able to parse the group wire format and
 treat group fields as unknown fields.


 	�

 	�
-
 
�" Length-delimited aggregate.


 
�

 
�
#
 � New in version 2.


 �

 �

 �

 �

 �

 �

 �

 �

 �

 �

 �

 �

 �

 �
'
 �" Uses ZigZag encoding.


 �

 �
'
 �" Uses ZigZag encoding.


 �

 �

��

�
*
 � 0 is reserved for errors


 �

 �

�

�

�

�

�

�

 �

 �


 �

 �

 �

�

�


�

�

�

�

�


�

�

�
�
�� If type_name is set, this need not be set.  If both this and type_name
 are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.


�


�

�

�
�
� � For message and enum types, this is the name of the type.  If the name
 starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
 rules are used to find the type (i.e. first the nested types within this
 message are searched, then within the parent, on up to the root
 namespace).


�


�

�

�
~
�p For extensions, this is the name of the type being extended.  It is
 resolved in the same manner as type_name.


�


�

�

�
�
�$� For numeric types, contains the original text representation of the value.
 For booleans, "true" or "false".
 For strings, contains the default text contents (not escaped in any way).
 For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
 TODO(kenton):  Base-64 encode?


�


�

�

�"#
�
�!v If set, gives the index of a oneof in the containing type's oneof_decl
 list.  This field is a member of that oneof.


�


�

�

� 
�
�!� JSON name of this field. The value is set by protocol compiler. If the
 user has set a "json_name" option on this field, that option's value
 will be used. Otherwise, it's deduced from the field's name by converting
 it to camelCase.


�


�

�

� 

	�$

	�


	�

	�

	�"#
�	

�%�	 If true, this is a proto3 "optional". When a proto3 field is optional, it
 tracks presence regardless of field type.

 When proto3_optional is true, this field must be belong to a oneof to
 signal to old proto3 clients that presence is tracked for this field. This
 oneof is known as a "synthetic" oneof, and this field must be its sole
 member (each proto3 optional field gets its own synthetic oneof). Synthetic
 oneofs exist in the descriptor only, and do not generate any API. Synthetic
 oneofs must be ordered after all "real" oneofs.

 For message fields, proto3_optional doesn't create any semantic change,
 since non-repeated message fields always track presence. However it still
 indicates the semantic detail of whether the user wrote "optional" or not.
 This can be useful for round-tripping the .proto file. For consistency we
 give message fields a synthetic oneof also, even though it is not required
 to track presence. This is especially important because the parser can't
 tell if a field is a message or an enum, so it must always create a
 synthetic oneof.

 Proto2 optional fields do not set this flag, because they already indicate
 optional with `LABEL_OPTIONAL`.



�



�


�


�"$
"
� � Describes a oneof.


�

 �

 �


 �

 �

 �

�$

�


�

�

�"#
'
� � Describes an enum type.


�

 �

 �


 �

 �

 �

�.

�


�#

�$)

�,-

�#

�


�

�

�!"
�
 ��� Range of reserved numeric values. Reserved values may not be used by
 entries in the same enum. Reserved ranges may not overlap.

 Note that this is distinct from DescriptorProto.ReservedRange in that it
 is inclusive such that it can appropriately represent the entire int32
 domain.


 �


  �" Inclusive.


  �

  �

  �

  �

 �" Inclusive.


 �

 �

 �

 �
�
�0� Range of reserved numeric values. Reserved numeric values may not be used
 by enum values in the same enum declaration. Reserved ranges may not
 overlap.


�


�

�+

�./
l
�$^ Reserved enum value names, which may not be reused. A given name may only
 be reserved once.


�


�

�

�"#
1
� �# Describes a value within an enum.


� 

 �

 �


 �

 �

 �

�

�


�

�

�

�(

�


�

�#

�&'
$
� � Describes a service.


�

 �

 �


 �

 �

 �

�,

�


� 

�!'

�*+

�&

�


�

�!

�$%
0
	� �" Describes a method of a service.


	�

	 �

	 �


	 �

	 �

	 �
�
	�!� Input and output type names.  These are resolved in the same way as
 FieldDescriptorProto.type_name, but must refer to a message type.


	�


	�

	�

	� 

	�"

	�


	�

	�

	� !

	�%

	�


	�

	� 

	�#$
E
	�77 Identifies if client streams multiple client messages


	�


	�

	� 

	�#$

	�%6

	�05
E
	�77 Identifies if server streams multiple server messages


	�


	�

	� 

	�#$

	�%6

	�05
�

� �2N ===================================================================
 Options
2� Each of the definitions above may have "options" attached.  These are
 just annotations which may cause code to be generated slightly differently
 or may contain hints for code that manipulates protocol messages.

 Clients may define custom options as extensions of the *Options messages.
 These extensions may not yet be known at parsing time, so the parser cannot
 store the values in them.  Instead it stores them in a field in the *Options
 message called uninterpreted_option. This field must have the same name
 across all *Options messages. We then use this field to populate the
 extensions when we build a descriptor, at which point all protos have been
 parsed and so all extensions are known.

 Extension numbers for custom options may be chosen as follows:
 * For options which will only be used within a single application or
   organization, or for experimental options, use field numbers 50000
   through 99999.  It is up to you to ensure that you do not use the
   same number for multiple options.
 * For options which will be published and used publicly by multiple
   independent entities, e-mail protobuf-global-extension-registry@google.com
   to reserve extension numbers. Simply provide your project name (e.g.
   Objective-C plugin) and your project website (if available) -- there's no
   need to explain how you intend to use them. Usually you only need one
   extension number. You can declare multiple options with only one extension
   number by putting them in a sub-message. See the Custom Options section of
   the docs for examples:
   https://developers.google.com/protocol-buffers/docs/proto#options
   If this turns out to be popular, a web service will be set up
   to automatically assign option numbers.



�
�

 �#� Sets the Java package where classes generated from this .proto will be
 placed.  By default, the proto package is used, but this is often
 inappropriate because proto packages do not normally start with backwards
 domain names.



 �



 �


 �


 �!"
�

�+� Controls the name of the wrapper Java class generated for the .proto file.
 That class will always contain the .proto file's getDescriptor() method as
 well as any top-level extensions defined in the .proto file.
 If java_multiple_files is disabled, then all the other classes from the
 .proto file will be nested inside the single wrapper outer class.



�



�


�&


�)*
�

�;� If enabled, then the Java code generator will generate a separate .java
 file for each top-level message, enum, and service defined in the .proto
 file.  Thus, these types will *not* be nested inside the wrapper class
 named by java_outer_classname.  However, the wrapper class will still be
 generated to contain the file's getDescriptor() method as well as any
 top-level extensions defined in the file.



�



�


�#


�&(


�):


�49
)

�E This option does nothing.



�



�


�-


�02


�3D


�4C
�

�>� If set true, then the Java2 code generator will generate code that
 throws an exception whenever an attempt is made to assign a non-UTF-8
 byte sequence to a string field.
 Message reflection will do the same.
 However, an extension field still accepts non-UTF-8 byte sequences.
 This option has no effect on when used with the lite runtime.



�



�


�&


�)+


�,=


�7<
L

 ��< Generated classes can be optimized for speed or code size.



 �
D

  �"4 Generate complete code for parsing, serialization,



  �	


  �
G

 � etc.
"/ Use ReflectionOps to implement these methods.



 �


 �
G

 �"7 Generate code using MessageLite and the lite runtime.



 �


 �


�;


�



�


�$


�'(


�):


�49
�

�"� Sets the Go package where structs generated from this .proto will be
 placed. If omitted, the Go package will be derived from the following:
   - The basename of the package import path, if provided.
   - Otherwise, the package statement in the .proto file, if present.
   - Otherwise, the basename of the .proto file, without extension.



�



�


�


�!
�

�;� Should generic services be generated in each language?  "Generic" services
 are not specific to any particular RPC system.  They are generated by the
 main code generators in each language (without additional plugins).
 Generic services were the only kind of service generation supported by
 early versions of google.protobuf.

 Generic services are now considered deprecated in favor of using plugins
 that generate code specific to your particular RPC system.  Therefore,
 these default to false.  Old code which depends on generic services should
 explicitly set them to true.



�



�


�#


�&(


�):


�49


�=


�



�


�%


�(*


�+<


�6;


	�;


	�



	�


	�#


	�&(


	�):


	�49



�<



�




�



�$



�')



�*;



�5:
�

�2� Is this file deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for everything in the file, or it will be completely ignored; in the very
 least, this is a formalization for deprecating files.



�



�


�


�


� 1


�+0


�7q Enables the use of arenas for the proto messages in this file. This applies
 only to generated classes for C++.



�



�


� 


�#%


�&6


�15
�

�)� Sets the objective c class prefix which is prepended to all objective c
 generated classes from this .proto. There is no default.



�



�


�#


�&(
I

�(; Namespace for generated classes; defaults to the package.



�



�


�"


�%'
�

�$� By default Swift generators will take the proto package and CamelCase it
 replacing '.' with underscore and use that to prefix the types/symbols
 defined. When this options is provided, they will use this value instead
 to prefix the types/symbols defined.



�



�


�


�!#
~

�(p Sets the php class prefix which is prepended to all php generated classes
 from this .proto. Default is empty.



�



�


�"


�%'
�

�%� Use this option to change the namespace of php generated classes. Default
 is empty. When this option is empty, the package name will be used for
 determining the namespace.



�



�


�


�"$
�

�.� Use this option to change the namespace of php generated metadata classes.
 Default is empty. When this option is empty, the proto file name will be
 used for determining the namespace.



�



�


�(


�+-
�

�$� Use this option to change the package of ruby generated classes. Default
 is empty. When this option is not set, the package name will be used for
 determining the ruby package.



�



�


�


�!#
|

�:n The parser stores options it doesn't recognize here.
 See the documentation for the "Options" section above.



�



�


�3


�69
�

�z Clients can define custom options in extensions of this message.
 See the documentation for the "Options" section above.



 �


 �


 �


	�


	 �


	 �


	 �

� �

�
�
 �>� Set true to use the old proto1 MessageSet wire format for extensions.
 This is provided for backwards-compatibility with the MessageSet wire
 format.  You should not use this for any other reason:  It's less
 efficient, has fewer features, and is more complicated.

 The message must be defined exactly as follows:
   message Foo {
     option message_set_wire_format = true;
     extensions 4 to max;
   }
 Note that the message cannot have any defined fields; MessageSets only
 have extensions.

 All extensions of your type must be singular messages; e.g. they cannot
 be int32s, enums, or repeated messages.

 Because this is an option, the above two restrictions are not enforced by
 the protocol compiler.


 �


 �

 �'

 �*+

 �,=

 �7<
�
�F� Disables the generation of the standard "descriptor()" accessor, which can
 conflict with a field of the same name.  This is meant to make migration
 from proto1 easier; new code should avoid fields named "descriptor".


�


�

�/

�23

�4E

�?D
�
�1� Is this message deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for the message, or it will be completely ignored; in the very least,
 this is a formalization for deprecating messages.


�


�

�

�

�0

�*/

	�

	 �

	 �

	 �

	�

	�

	�

	�

	�

	�
�
�� Whether the message is an automatically generated map entry type for the
 maps field.

 For maps fields:
     map<KeyType, ValueType> map_field = 1;
 The parsed descriptor looks like:
     message MapFieldEntry {
         option map_entry = true;
         optional KeyType key = 1;
         optional ValueType value = 2;
     }
     repeated MapFieldEntry map_field = 1;

 Implementations may choose not to generate the map_entry=true message, but
 use a native map in the target language to hold the keys and values.
 The reflection APIs in such implementations still need to work as
 if the field is a repeated message field.

 NOTE: Do not set the option in .proto files. Always use the maps syntax
 instead. The option should only be implicitly set by the proto compiler
 parser.


�


�

�

�
$
	�" javalite_serializable


	�

	�

	�

	�" javanano_as_lite


	�

	�

	�
O
�:A The parser stores options it doesn't recognize here. See above.


�


�

�3

�69
Z
�M Clients can define custom options in extensions of this message. See above.


 �

 �

 �

� �

�
�
 �.� The ctype option instructs the C++ code generator to use a different
 representation of the field than it normally would.  See the specific
 options below.  This option is not yet implemented in the open source
 release -- sorry, we'll try to include it in a future version!


 �


 �

 �

 �

 �-

 �&,

 ��

 �

  � Default mode.


  �


  �

 �

 �

 �

 �

 �

 �
�
�� The packed option can be enabled for repeated primitive fields to enable
 a more efficient representation on the wire. Rather than repeatedly
 writing the tag and type for each element, the entire array is encoded as
 a single length-delimited blob. In proto3, only explicit setting it to
 false will avoid using packed encoding.


�


�

�

�
�
�3� The jstype option determines the JavaScript type used for values of the
 field.  The option is permitted only for 64 bit integral and fixed types
 (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
 is represented as JavaScript string, which avoids loss of precision that
 can happen when a large value is converted to a floating point JavaScript.
 Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
 use the JavaScript "number" type.  The behavior of the default option
 JS_NORMAL is implementation dependent.

 This option is an enum to permit additional types to be added, e.g.
 goog.math.Integer.


�


�

�

�

�2

�(1

��

�
'
 � Use the default type.


 �

 �
)
� Use JavaScript strings.


�

�
)
� Use JavaScript numbers.


�

�
�
�+� Should this field be parsed lazily?  Lazy applies only to message-type
 fields.  It means that when the outer message is initially parsed, the
 inner message's contents will not be parsed but instead stored in encoded
 form.  The inner message will actually be parsed when it is first accessed.

 This is only a hint.  Implementations are free to choose whether to use
 eager or lazy parsing regardless of the value of this option.  However,
 setting this option true suggests that the protocol author believes that
 using lazy parsing on this field is worth the additional bookkeeping
 overhead typically needed to implement it.

 This option does not affect the public interface of any generated code;
 all method signatures remain the same.  Furthermore, thread-safety of the
 interface is not affected by this option; const methods remain safe to
 call from multiple threads concurrently, while non-const methods continue
 to require exclusive access.


 Note that implementations may choose not to check required fields within
 a lazy sub-message.  That is, calling IsInitialized() on the outer message
 may return true even if the inner message has missing required fields.
 This is necessary because otherwise the inner message would have to be
 parsed in order to perform the check, defeating the purpose of lazy
 parsing.  An implementation which chooses not to check required fields
 must be consistent about it.  That is, for any particular sub-message, the
 implementation must either *always* check its required fields, or *never*
 check its required fields, regardless of whether or not the message has
 been parsed.


�


�

�

�

�*

�$)
�
�1� Is this field deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for accessors, or it will be completely ignored; in the very least, this
 is a formalization for deprecating fields.


�


�

�

�

�0

�*/
?
�,1 For Google-internal migration only. Do not use.


�


�

�

�

�+

�%*
O
�:A The parser stores options it doesn't recognize here. See above.


�


�

�3

�69
Z
�M Clients can define custom options in extensions of this message. See above.


 �

 �

 �

	�" removed jtype


	 �

	 �

	 �

� �

�
O
 �:A The parser stores options it doesn't recognize here. See above.


 �


 �

 �3

 �69
Z
�M Clients can define custom options in extensions of this message. See above.


 �

 �

 �

� �

�
`
 � R Set this option to true to allow mapping different tag names to the same
 value.


 �


 �

 �

 �
�
�1� Is this enum deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for the enum, or it will be completely ignored; in the very least, this
 is a formalization for deprecating enums.


�


�

�

�

�0

�*/

	�" javanano_as_lite


	 �

	 �

	 �
O
�:A The parser stores options it doesn't recognize here. See above.


�


�

�3

�69
Z
�M Clients can define custom options in extensions of this message. See above.


 �

 �

 �

� �

�
�
 �1� Is this enum value deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for the enum value, or it will be completely ignored; in the very least,
 this is a formalization for deprecating enum values.


 �


 �

 �

 �

 �0

 �*/
O
�:A The parser stores options it doesn't recognize here. See above.


�


�

�3

�69
Z
�M Clients can define custom options in extensions of this message. See above.


 �

 �

 �

� �

�
�
 �2� Is this service deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for the service, or it will be completely ignored; in the very least,
 this is a formalization for deprecating services.
2� Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
   framework.  We apologize for hoarding these numbers to ourselves, but
   we were already using them long before we decided to release Protocol
   Buffers.


 �


 �

 �

 �

 � 1

 �+0
O
�:A The parser stores options it doesn't recognize here. See above.


�


�

�3

�69
Z
�M Clients can define custom options in extensions of this message. See above.


 �

 �

 �

� �

�
�
 �2� Is this method deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for the method, or it will be completely ignored; in the very least,
 this is a formalization for deprecating methods.
2� Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
   framework.  We apologize for hoarding these numbers to ourselves, but
   we were already using them long before we decided to release Protocol
   Buffers.


 �


 �

 �

 �

 � 1

 �+0
�
 ��� Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
 or neither? HTTP based RPC implementation may choose GET verb for safe
 methods, and PUT verb for idempotent methods instead of the default POST.


 �

  �

  �

  �
$
 �" implies idempotent


 �

 �
7
 �"' idempotent, but may have side effects


 �

 �

��&

�


�

�-

�02

�%

�$
O
�:A The parser stores options it doesn't recognize here. See above.


�


�

�3

�69
Z
�M Clients can define custom options in extensions of this message. See above.


 �

 �

 �
�
� �� A message representing a option the parser does not recognize. This only
 appears in options protos created by the compiler::Parser class.
 DescriptorPool resolves these when building Descriptor objects. Therefore,
 options protos in descriptor objects (e.g. returned by Descriptor::options(),
 or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
 in them.


�
�
 ��� The name of the uninterpreted option.  Each string represents a segment in
 a dot-separated name.  is_extension is true iff a segment represents an
 extension (denoted with parentheses in options specs in .proto files).
 E.g.,{ ["foo", false], ["bar.baz", true], ["qux", false] } represents
 "foo.(bar.baz).qux".


 �


  �"

  �

  �

  �

  � !

 �#

 �

 �

 �

 �!"

 �

 �


 �

 �

 �
�
�'� The value of the uninterpreted option, in whatever type the tokenizer
 identified it as during parsing. Exactly one of these should be set.


�


�

�"

�%&

�)

�


�

�$

�'(

�(

�


�

�#

�&'

�#

�


�

�

�!"

�"

�


�

�

� !

�&

�


�

�!

�$%
�
� �j Encapsulates information about the original source file from which a
 FileDescriptorProto was generated.
2` ===================================================================
 Optional source code info


�
�
 �!� A Location identifies a piece of source code in a .proto file which
 corresponds to a particular definition.  This information is intended
 to be useful to IDEs, code indexers, documentation generators, and similar
 tools.

 For example, say we have a file like:
   message Foo {
     optional string foo = 1;
   }
 Let's look at just the field definition:
   optional string foo = 1;
   ^       ^^     ^^  ^  ^^^
   a       bc     de  f  ghi
 We have the following locations:
   span   path               represents
   [a,i)  [ 4, 0, 2, 0 ]     The whole field definition.
   [a,b)  [ 4, 0, 2, 0, 4 ]  The label (optional).
   [c,d)  [ 4, 0, 2, 0, 5 ]  The type (string).
   [e,f)  [ 4, 0, 2, 0, 1 ]  The name (foo).
   [g,h)  [ 4, 0, 2, 0, 3 ]  The number (1).

 Notes:
 - A location may refer to a repeated field itself (i.e. not to any
   particular index within it).  This is used whenever a set of elements are
   logically enclosed in a single code segment.  For example, an entire
   extend block (possibly containing multiple extension definitions) will
   have an outer location whose path refers to the "extensions" repeated
   field without an index.
 - Multiple locations may have the same path.  This happens when a single
   logical declaration is spread out across multiple places.  The most
   obvious example is the "extend" block again -- there may be multiple
   extend blocks in the same scope, each of which will have the same path.
 - A location's span is not always a subset of its parent's span.  For
   example, the "extendee" of an extension declaration appears at the
   beginning of the "extend" block and is shared by all extensions within
   the block.
 - Just because a location's span is a subset of some other location's span
   does not mean that it is a descendant.  For example, a "group" defines
   both a type and a field in a single declaration.  Thus, the locations
   corresponding to the type and field and their components will overlap.
 - Code which tries to interpret locations should probably be designed to
   ignore those that it doesn't understand, as more types of locations could
   be recorded in the future.


 �


 �

 �

 � 

 ��

 �

�
  �,� Identifies which part of the FileDescriptorProto was defined at this
 location.

 Each element is a field number or an index.  They form a path from
 the root FileDescriptorProto to the place where the definition.  For
 example, this path:
   [ 4, 3, 2, 7, 1 ]
 refers to:
   file.message_type(3)  // 4, 3
       .field(7)         // 2, 7
       .name()           // 1
 This is because FileDescriptorProto.message_type has field number 4:
   repeated DescriptorProto message_type = 4;
 and DescriptorProto.field has field number 2:
   repeated FieldDescriptorProto field = 2;
 and FieldDescriptorProto.name has field number 1:
   optional string name = 1;

 Thus, the above path gives the location of a field name.  If we removed
 the last element:
   [ 4, 3, 2, 7 ]
 this path refers to the whole field declaration (from the beginning
 of the label to the terminating semicolon).


  �

  �

  �

  �

  �+

  �*
�
 �,� Always has exactly three or four elements: start line, start column,
 end line (optional, otherwise assumed same as start line), end column.
 These are packed into a single field for efficiency.  Note that line
 and column numbers are zero-based -- typically you will want to add
 1 to each before displaying to a user.


 �

 �

 �

 �

 �+

 �*
�
 �)� If this SourceCodeInfo represents a complete declaration, these are any
 comments appearing before and after the declaration which appear to be
 attached to the declaration.

 A series of line comments appearing on consecutive lines, with no other
 tokens appearing on those lines, will be treated as a single comment.

 leading_detached_comments will keep paragraphs of comments that appear
 before (but not connected to) the current element. Each paragraph,
 separated by empty lines, will be one comment element in the repeated
 field.

 Only the comment content is provided; comment markers (e.g. //) are
 stripped out.  For block comments, leading whitespace and an asterisk
 will be stripped from the beginning of each line other than the first.
 Newlines are included in the output.

 Examples:

   optional int32 foo = 1;  // Comment attached to foo.
   // Comment attached to bar.
   optional int32 bar = 2;

   optional string baz = 3;
   // Comment attached to baz.
   // Another line attached to baz.

   // Comment attached to qux.
   //
   // Another line attached to qux.
   optional double qux = 4;

   // Detached comment for corge. This is not leading or trailing comments
   // to qux or corge because there are blank lines separating it from
   // both.

   // Detached comment for corge paragraph 2.

   optional string corge = 5;
   /* Block comment attached
    * to corge.  Leading asterisks
    * will be removed. */
   /* Block comment attached to
    * grault. */
   optional int32 grault = 6;

   // ignored detached comments.


 �

 �

 �$

 �'(

 �*

 �

 �

 �%

 �()

 �2

 �

 �

 �-

 �01
�
� �� Describes the relationship between generated code and its original source
 file. A GeneratedCodeInfo message is associated with only one generated
 source file, but may contain references to different source .proto files.


�
x
 �%j An Annotation connects some span of text in generated code to an element
 of its generating .proto file.


 �


 �

 � 

 �#$

 ��

 �

�
  �, Identifies the element in the original source .proto file. This field
 is formatted the same as SourceCodeInfo.Location.path.


  �

  �

  �

  �

  �+

  �*
O
 �$? Identifies the filesystem path to the original source .proto.


 �

 �

 �

 �"#
w
 �g Identifies the starting offset in bytes in the generated code
 that relates to the identified object.


 �

 �

 �

 �
�
 �� Identifies the ending offset in bytes in the generated code that
 relates to the identified offset. The end offset should be one past
 the last relevant byte (so the length of the text = end - begin).


 �

 �

 �

 �
�
google/api/annotations.proto
google.apigoogle/api/http.proto google/protobuf/descriptor.proto:K
http.google.protobuf.MethodOptions�ʼ" (2.google.api.HttpRuleRhttpBn
com.google.apiBAnnotationsProtoPZAgoogle.golang.org/genproto/googleapis/api/annotations;annotations�GAPIJ�
 
�
 2� Copyright 2015 Google LLC

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.


 
	
  
	
 *

 X
	
 X

 "
	

 "

 1
	
 1

 '
	
 '

 "
	
$ "
	
 

  See `HttpRule`.



 $


 



 


 bproto3
�,
google/protobuf/any.protogoogle.protobuf"6
Any
type_url (	RtypeUrl
value (RvalueBv
com.google.protobufBAnyProtoPZ,google.golang.org/protobuf/types/known/anypb�GPB�Google.Protobuf.WellKnownTypesJ�*
 �
�
 2� Protocol Buffers - Google's data interchange format
 Copyright 2008 Google Inc.  All rights reserved.
 https://developers.google.com/protocol-buffers/

 Redistribution and use in source and binary forms, with or without
 modification, are permitted provided that the following conditions are
 met:

     * Redistributions of source code must retain the above copyright
 notice, this list of conditions and the following disclaimer.
     * Redistributions in binary form must reproduce the above
 copyright notice, this list of conditions and the following disclaimer
 in the documentation and/or other materials provided with the
 distribution.
     * Neither the name of Google Inc. nor the names of its
 contributors may be used to endorse or promote products derived from
 this software without specific prior written permission.

 THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


  

" ;
	
%" ;

# C
	
# C

$ ,
	
$ ,

% )
	
% )

& "
	

& "

' !
	
$' !
�
 | �� `Any` contains an arbitrary serialized protocol buffer message along with a
 URL that describes the type of the serialized message.

 Protobuf library provides support to pack/unpack Any values in the form
 of utility functions or additional generated methods of the Any type.

 Example 1: Pack and unpack a message in C++.

     Foo foo = ...;
     Any any;
     any.PackFrom(foo);
     ...
     if (any.UnpackTo(&foo)) {
       ...
     }

 Example 2: Pack and unpack a message in Java.

     Foo foo = ...;
     Any any = Any.pack(foo);
     ...
     if (any.is(Foo.class)) {
       foo = any.unpack(Foo.class);
     }

  Example 3: Pack and unpack a message in Python.

     foo = Foo(...)
     any = Any()
     any.Pack(foo)
     ...
     if any.Is(Foo.DESCRIPTOR):
       any.Unpack(foo)
       ...

  Example 4: Pack and unpack a message in Go

      foo := &pb.Foo{...}
      any, err := anypb.New(foo)
      if err != nil {
        ...
      }
      ...
      foo := &pb.Foo{}
      if err := any.UnmarshalTo(foo); err != nil {
        ...
      }

 The pack methods provided by protobuf library will by default use
 'type.googleapis.com/full.type.name' as the type URL and the unpack
 methods only use the fully qualified type name after the last '/'
 in the type URL, for example "foo.bar.com/x/y.z" will yield type
 name "y.z".


 JSON
 ====
 The JSON representation of an `Any` value uses the regular
 representation of the deserialized, embedded message, with an
 additional field `@type` which contains the type URL. Example:

     package google.profile;
     message Person {
       string first_name = 1;
       string last_name = 2;
     }

     {
       "@type": "type.googleapis.com/google.profile.Person",
       "firstName": <string>,
       "lastName": <string>
     }

 If the embedded message type is well-known and has a custom JSON
 representation, that representation will be embedded adding a field
 `value` which holds the custom JSON in addition to the `@type`
 field. Example (for message [google.protobuf.Duration][]):

     {
       "@type": "type.googleapis.com/google.protobuf.Duration",
       "value": "1.212s"
     }




 |
�

  ��
 A URL/resource name that uniquely identifies the type of the serialized
 protocol buffer message. This string must contain at least
 one "/" character. The last segment of the URL's path must represent
 the fully qualified name of the type (as in
 `path/google.protobuf.Duration`). The name should be in a canonical form
 (e.g., leading "." is not accepted).

 In practice, teams usually precompile into the binary all types that they
 expect it to use in the context of Any. However, for URLs which use the
 scheme `http`, `https`, or no scheme, one can optionally set up a type
 server that maps type URLs to message definitions as follows:

 * If no scheme is provided, `https` is assumed.
 * An HTTP GET on the URL must yield a [google.protobuf.Type][]
   value in binary format, or produce an error.
 * Applications are allowed to cache lookup results based on the
   URL, or have them precompiled into a binary to avoid any
   lookup. Therefore, binary compatibility needs to be preserved
   on changes to types. (Use versioned type names to manage
   breaking changes.)

 Note: this functionality is not currently available in the official
 protobuf release, and it is not used for type URLs beginning with
 type.googleapis.com.

 Schemes other than `http`, `https` (or the empty scheme) might be
 used with implementation specific semantics.



  �

  �	

  �
W
 �I Must be a valid serialized protocol buffer of the above specified type.


 �

 �

 �bproto3
�
google/api/httpbody.proto
google.apigoogle/protobuf/any.proto"w
HttpBody!
content_type (	RcontentType
data (Rdata4

extensions (2.google.protobuf.AnyR
extensionsBh
com.google.apiBHttpBodyProtoPZ;google.golang.org/genproto/googleapis/api/httpbody;httpbody��GAPIJ�
 P
�
 2� Copyright 2015 Google LLC

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.


 
	
  #

 
	
 

 R
	
 R

 "
	

 "

 .
	
 .

 '
	
 '

 "
	
$ "
�

 F P�
 Message that represents an arbitrary HTTP body. It should only be used for
 payload formats that can't be represented as JSON, such as raw binary or
 an HTML page.


 This message can be used both in streaming and non-streaming API methods in
 the request as well as the response.

 It can be used as a top-level request field, which is convenient if one
 wants to extract parameters from either the URL or HTTP template into the
 request fields and also want access to the raw HTTP body.

 Example:

     message GetResourceRequest {
       // A unique request id.
       string request_id = 1;

       // The raw HTTP body is bound to this field.
       google.api.HttpBody http_body = 2;

     }

     service ResourceService {
       rpc GetResource(GetResourceRequest)
         returns (google.api.HttpBody);
       rpc UpdateResource(google.api.HttpBody)
         returns (google.protobuf.Empty);

     }

 Example with streaming methods:

     service CaldavService {
       rpc GetCalendar(stream google.api.HttpBody)
         returns (stream google.api.HttpBody);
       rpc UpdateCalendar(stream google.api.HttpBody)
         returns (stream google.api.HttpBody);

     }

 Use of this type only changes how the request and response bodies are
 handled, all other features will continue to work unchanged.



 F
Z
  HM The HTTP Content-Type header value specifying the content type of the body.


  H

  H	

  H
<
 K/ The HTTP request/response body as raw binary.


 K

 K

 K
m
 O.` Application specific response metadata. Must be set in the first response
 for streaming APIs.


 O


 O

 O)

 O,-bproto3
�
httpcache.protocache_clientcacheclient.protogoogle/api/httpbody.protogoogle/api/annotations.proto"c
_HttpGetRequest

cache_name (	R	cacheName
	cache_key (	RcacheKey
token (	Rtoken"�
_HttpSetRequest

cache_name (	R	cacheName
	cache_key (	RcacheKey)
ttl_milliseconds (RttlMilliseconds
token (	Rtoken3

cache_body (2.google.api.HttpBodyR	cacheBody2�
	HttpCacheg
Get.cache_client._HttpGetRequest.google.api.HttpBody"+���%#/cache/get/{cache_name}/{cache_key}y
Set.cache_client._HttpSetRequest.cache_client._SetResponse"7���1"#/cache/set/{cache_name}/{cache_key}:
cache_body�
SetButItsAPut.cache_client._HttpSetRequest.cache_client._SetResponse"7���1#/cache/set/{cache_name}/{cache_key}:
cache_bodyBG
grpc.cache_clientPZ0github.com/momentohq/client-sdk-go;client_sdk_goJ�
  /

  

 G
	
 G

 "
	

 "

 *
	
 *

 
	
  
	
	 #
	

 &


  


 

  

  

  	

  

 

 

 	

 

 

 

 	

 


 




 

 

 	

 





	







	







	



%



 

#$


  /


 

   

  	

  

  %8

  

	  �ʼ"

 !'

 !	

 !

 !%1

 "&

	 �ʼ""&

 (.

 (

 ($

 (/;

 )-

	 �ʼ")-bproto3